mod commands;

use std::process::Child;
use std::sync::Arc;

use config::Config;
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::{async_trait, Client};

lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("server-man-config"))
        .build()
        .unwrap();
}

fn parse_server_configs(config: &Config) -> Vec<ServerConfig> {
   config 
       .get_array("servers")
       .expect("Could not find 'servers' array in config")
       .iter()
       .map(|val| {
           val.clone()
               .try_deserialize::<ServerConfig>()
               .expect("Could not deserialize 'servers' array")
       })
   .collect()
}

struct ServerProcessMap;
impl TypeMapKey for ServerProcessMap {
    type Value = Arc<RwLock<Vec<Option<Child>>>>;
}

fn default_java() -> String {
    "java".to_string()
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ServerConfig {
    #[serde(rename_all = "kebab-case")]
    JavaConfig {
        name: String,
        dir: String,
        server_jar: String,
        max_mem: String,
        min_mem: String,
        #[serde(default = "default_java")]
        java: String,
        #[serde(default)]
        extra_opts: String,
        guild_id: u64,
    },
    #[serde(rename_all = "kebab-case")]
    BedrockConfig {
        name: String,
        dir: String,
        exe: String,
        guild_id: u64,
    },
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "list" => commands::list::run(&command, &parse_server_configs(&CONFIG)),
                "start" => {
                    let proc_map = {
                        let data_read = ctx.data.read().await;
                        data_read
                            .get::<ServerProcessMap>()
                            .expect("Expected ServerProcessMap in TypeMap.")
                            .clone()
                    };

                    {
                        let mut server_process = proc_map.write().await;
                        commands::start::run(
                            &command,
                            &command.data.options,
                            &mut *server_process,
                            public_ip::addr().await,
                            &CONFIG,
                        )
                    }
                }
                "stop" => {
                    let proc_map = {
                        let data_read = ctx.data.read().await;
                        data_read
                            .get::<ServerProcessMap>()
                            .expect("Expected ServerProcessMap in TypeMap.")
                            .clone()
                    };

                    {
                        let mut server_process = proc_map.write().await;
                        commands::stop::run(
                            &command,
                            &command.data.options,
                            &mut *server_process,
                            &CONFIG,
                        )
                    }
                }
                other => format!("not implemented: {other}"),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let server_configs = parse_server_configs(&CONFIG);

        for server_config in &server_configs {
            let guild_id = GuildId(match server_config {
                ServerConfig::JavaConfig { guild_id, .. } => *guild_id,
                ServerConfig::BedrockConfig { guild_id, .. } => *guild_id,
            });

            let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::list::register(command))
                    .create_application_command(|command| {
                        commands::start::register(command, &guild_id, &server_configs)
                    })
                    .create_application_command(|command| {
                        commands::stop::register(command, &guild_id, &server_configs)
                    })
            })
            .await;

            println!(
                "Registered following commands for {:?}: {:?}",
                guild_id, commands
            );
        }
    }
}

#[tokio::main]
async fn main() {
    let token = CONFIG.get::<String>("token").unwrap();

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let num_servers = CONFIG
        .get_array("servers")
        .expect("Expected servers array in config")
        .len();

    {
        // Get a write lock to the data in the client
        // Use a block to make sure the lock is released
        let mut proc_map = Vec::with_capacity(num_servers);
        for _ in 0..num_servers {
            proc_map.push(None);
        }
        let mut data = client.data.write().await;
        data.insert::<ServerProcessMap>(Arc::new(RwLock::new(proc_map)));
    }

    if let Err(why) = client.start().await {
        println!("client error: {:?}", why);
    }
}
