use std::io::Write;
use std::process::Child;

use config::Config;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
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
    config: &Config,
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

            if let Ok(servers) = config.get_array("servers") {
                servers.iter().enumerate().for_each(|(index, val)| {
                    let server_name = val.clone().try_deserialize::<ServerConfig>().map_or(
                        "Failed to deserialize server config".to_string(),
                        |server_config| server_config.name,
                    );
                    option.add_int_choice(server_name, index as i32);
                })
            } else {
                println!("No 'servers' array found in config!");
            }

            option
        })
}
