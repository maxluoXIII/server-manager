use std::net::IpAddr;
use std::process::{Child, Command, Stdio};

use config::Config;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::{parse_server_configs, ServerConfig};

fn start_server(server_config: &ServerConfig) -> std::io::Result<Child> {
    match server_config {
        ServerConfig::JavaConfig {
            dir,
            server_jar,
            max_mem,
            min_mem,
            java,
            extra_opts,
            ..
        } => Command::new(java)
            .args([
                format!("-Xmx{}", max_mem).as_str(),
                format!("-Xms{}", min_mem).as_str(),
            ])
            .args(extra_opts.split_whitespace())
            .args(["-jar", &server_jar, "nogui"])
            .current_dir(dir)
            .stdin(Stdio::piped())
            .spawn(),

        ServerConfig::BedrockConfig { dir, exe, .. } => Command::new(format!("{dir}/{exe}"))
            .current_dir(dir)
            .stdin(Stdio::piped())
            .spawn(),
    }
}

pub fn run(
    command: &ApplicationCommandInteraction,
    options: &[CommandDataOption],
    proc_map: &mut Vec<Option<Child>>,
    ip: Option<IpAddr>,
    config: &Config,
) -> String {
    if command.guild_id.is_none() {
        return "No guild id in command".to_string();
    }

    let notify_id = config
        .get::<u64>("notify-id")
        .expect("Expected required notify id");

    let server_index = match options
        .iter()
        .find(|option| option.name == "server")
        .expect("Expected required server index option")
        .resolved
        .clone()
        .expect("Expected required server option value")
    {
        CommandDataOptionValue::Integer(index) => index as usize,
        _ => return "Did not receive a server index".to_string(),
    };

    let server_configs = parse_server_configs(config);
    let server_config = server_configs
        .get(server_index)
        .expect(&format!("Could not get config for index {server_index}"));
    if match server_config {
        ServerConfig::JavaConfig { guild_id, .. } => GuildId(*guild_id) != command.guild_id.unwrap(),
        ServerConfig::BedrockConfig { guild_id, .. } => GuildId(*guild_id) != command.guild_id.unwrap(),
    } {
        return "Error: trying to control server from wrong guild".to_string();
    }

    if server_index >= proc_map.len() {
        return "Server index out of bounds".to_string();
    }
    let server_proc = &mut proc_map[server_index];

    match *server_proc {
        Some(_) => "This server is already running!".to_string(),
        None => {
            let server_config = match config
                .get_array("servers")
                .expect("Expected servers array in config")
                .get(server_index)
                .expect("Expected valid server index")
                .clone()
                .try_deserialize::<ServerConfig>()
            {
                Ok(server_config) => server_config,
                Err(e) => return format!("<@{notify_id}> {e}"),
            };

            match start_server(&server_config) {
                Ok(proc) => {
                    *server_proc = Some(proc);
                    if let Some(ip) = ip {
                        format!("<@{notify_id}> Starting the server at {ip}, please wait ~20 seconds before joining")
                    } else {
                        format!("<@{notify_id}> Starting the server, please wait ~20 seconds before joining")
                    }
                }
                Err(err) => {
                    format!("<@{notify_id}> Failed to start server process: {err}")
                }
            }
        }
    }
}

pub fn register<'a>(
    command: &'a mut CreateApplicationCommand,
    guild_id: &GuildId,
    server_configs: &Vec<ServerConfig>,
) -> &'a mut CreateApplicationCommand {
    command
        .name("start")
        .description("Start a minecraft server")
        .create_option(|option| {
            option
                .name("server")
                .description("Server index. See /list")
                .kind(CommandOptionType::Integer)
                .required(true);

            server_configs
                .iter()
                .enumerate()
                .filter(|(_, server_config)| {
                    *guild_id
                        == match server_config {
                            ServerConfig::JavaConfig { guild_id, .. } => GuildId(*guild_id),
                            ServerConfig::BedrockConfig { guild_id, .. } => GuildId(*guild_id),
                        }
                })
                .for_each(|(index, server_config)| {
                    let server_name = match server_config {
                        ServerConfig::JavaConfig { name, .. } => name,
                        ServerConfig::BedrockConfig { name, .. } => name,
                    };
                    println!("Added server {server_name}");
                    option.add_int_choice(server_name, index as i32);
                });

            option
        })
}
