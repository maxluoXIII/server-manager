use std::net::IpAddr;
use std::process::{Child, Command, Stdio};

use config::Config;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

use crate::ServerConfig;

pub fn run(
    options: &[CommandDataOption],
    proc_map: &mut Vec<Option<Child>>,
    ip: Option<IpAddr>,
    config: &Config,
) -> String {
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

    // Get information that server's process if it is running
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

            if let Ok(proc) = Command::new(server_config.java)
                .args([
                    format!("-Xmx{}", server_config.max_mem).as_str(),
                    format!("-Xms{}", server_config.min_mem).as_str(),
                ])
                .args(server_config.extra_opts.split_whitespace())
                .args(["-jar", &server_config.server_jar, "nogui"])
                .current_dir(server_config.dir)
                .stdin(Stdio::piped())
                .spawn()
            {
                *server_proc = Some(proc);
                if let Some(ip) = ip {
                    format!("<@{notify_id}> Starting the server at {ip}, please wait ~20 seconds before joining")
                } else {
                    format!("<@{notify_id}> Starting the server, please wait ~20 seconds before joining")
                }
            } else {
                format!("<@{notify_id}> Failed to start server process")
            }
        }
    }
}

pub fn register<'a>(
    command: &'a mut CreateApplicationCommand,
    config: &Config,
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

            if let Ok(servers) = config.get_array("servers") {
                servers.iter().enumerate().for_each(|(index, val)| {
                    let server_name = val.clone().try_deserialize::<ServerConfig>().map_or(
                        "Failed to deserialize server config".to_string(),
                        |server_config| server_config.name,
                    );
                    println!("Added server {server_name}");
                    option.add_int_choice(server_name, index as i32);
                })
            } else {
                println!("No 'servers' array found in config!");
            }

            option
        })
}
