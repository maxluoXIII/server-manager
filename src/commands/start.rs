use std::net::IpAddr;
use std::process::{Child, Command, Stdio};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::RwLockWriteGuard;

pub fn run(
    _options: &[CommandDataOption],
    mut server_proc: RwLockWriteGuard<Option<Child>>,
    ip: Option<IpAddr>,
    server_jar_name: &str,
    max_mem: &str,
    min_mem: &str,
    notify_id: u64,
) -> String {
    // Need to dereference twice to get through reference and lock
    match *server_proc {
        Some(_) => "Already running!".to_string(),
        None => {
            if let Ok(proc) = Command::new("java")
                .args([
                    format!("-Xmx{max_mem}").as_str(),
                    format!("-Xms{min_mem}").as_str(),
                    "-jar",
                    server_jar_name,
                    "nogui",
                ])
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
                format!("<@{notify_id}> Failed to start")
            }
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("start")
        .description("Start the minecraft server")
}
