use crate::ServerConfig;
use config::Config;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption], config: &Config) -> String {
    match config.get_array("servers") {
        Ok(servers) => servers
            .iter()
            .enumerate()
            .map(|(index, val)| {
                let server_name = val.clone().try_deserialize::<ServerConfig>().map_or(
                    "Failed to deserialize server config".to_string(),
                    |server_config| match server_config {
                        ServerConfig::JavaConfig { name, .. } => name,
                        ServerConfig::BedrockConfig { name, .. } => name,
                    },
                );
                format!("{index}. {server_name}\n")
            })
            .collect(),
        Err(e) => e.to_string(),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list")
        .description("List the servers that this bot manages")
}
