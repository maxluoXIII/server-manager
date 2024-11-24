use std::io::Write;
use std::process::Child;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::id::GuildId;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::ServerConfig;

pub fn run(
    options: &[CommandDataOption],
    proc_map: &mut Vec<Option<Child>>,
    notify_id: u64,
) -> String {
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

    if server_index >= proc_map.len() {
        return "Server index out of bounds".to_string();
    }
    let server_proc = &mut proc_map[server_index];

    if let Some(proc) = server_proc {
        let ret_string: String;
        if let Ok(_) = proc.stdin.as_mut().unwrap().write_all("stop".as_bytes()) {
            if let Ok(_) = proc.wait() {
                ret_string = format!("<@{notify_id}> Server has been stopped")
            } else {
                ret_string =
                    format!("<@{notify_id}> Tried to kill server, but it was already stopped")
            }
            *server_proc = None;
        } else {
            ret_string = format!("<@{notify_id}> Could not write 'stop' to stdin")
        }

        ret_string
    } else {
        "Server is already stopped!".to_string()
    }
}

pub fn register<'a>(
    command: &'a mut CreateApplicationCommand,
    guild_id: &GuildId,
    server_configs: &Vec<ServerConfig>,
) -> &'a mut CreateApplicationCommand {
    command
        .name("stop")
        .description("Stop a minecraft server")
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
                    option.add_int_choice(server_name, index as i32);
                });

            option
        })
}
