use crate::ServerConfig;
use serenity::builder::CreateApplicationCommand;
use serenity::model::{
    application::interaction::application_command::ApplicationCommandInteraction, id::GuildId,
};

pub fn run(command: &ApplicationCommandInteraction, server_configs: &Vec<ServerConfig>) -> String {
    if command.guild_id.is_none() {
        return "No guild id in command".to_string();
    }

    server_configs
        .iter()
        .filter(|server_config| match server_config {
            ServerConfig::JavaConfig { guild_id, .. } => {
                GuildId(*guild_id) == command.guild_id.unwrap()
            }
            ServerConfig::BedrockConfig { guild_id, .. } => {
                GuildId(*guild_id) == command.guild_id.unwrap()
            }
        })
        .enumerate()
        .map(|(index, server_config)| {
            let server_name = match server_config {
                ServerConfig::JavaConfig { name, .. } => name,
                ServerConfig::BedrockConfig { name, .. } => name,
            };
            format!("{index}. {server_name}\n")
        })
        .collect()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("list")
        .description("List the servers that this bot manages")
}
