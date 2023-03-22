use std::io::Write;
use std::process::Child;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tokio::sync::RwLockWriteGuard;

pub fn run(
    _options: &[CommandDataOption],
    server_proc: &mut RwLockWriteGuard<Option<Child>>,
    notify_id: u64,
) -> String {
    // Need to dereference twice to get through reference and lock
    // Then need to borrow again to get proc
    if let Some(proc) = &mut **server_proc {
        let ret_string: String;
        if let Ok(_) = proc.stdin.as_mut().unwrap().write_all("stop".as_bytes()) {
            if let Ok(_) = proc.wait() {
                ret_string = format!("<@{notify_id}> Server has been stopped")
            } else {
                ret_string =
                    format!("<@{notify_id}> Tried to kill server, but it was already stopped")
            }
            **server_proc = None;
        } else {
            ret_string = format!("<@{notify_id}> Could not write 'stop' to stdin")
        }

        ret_string
    } else {
        "Server is already stopped!".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("stop")
        .description("Stop the minecraft server")
}
