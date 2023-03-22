mod commands;

use std::process::Child;
use std::sync::Arc;

use config::Config;
use lazy_static::lazy_static;
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

struct ServerProcess;
impl TypeMapKey for ServerProcess {
    type Value = Arc<RwLock<Option<Child>>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "start" => {
                    let server_process = {
                        let data_read = ctx.data.read().await;
                        data_read
                            .get::<ServerProcess>()
                            .expect("Expected ServerProcess in TypeMap.")
                            .clone()
                    };

                    {
                        let server_process = server_process.write().await;
                        commands::start::run(
                            &command.data.options,
                            server_process,
                            public_ip::addr().await,
                            &CONFIG.get::<String>("server-jar").unwrap(),
                            &CONFIG.get::<String>("max-mem").unwrap(),
                            &CONFIG.get::<String>("min-mem").unwrap(),
                            CONFIG.get("notify-id").unwrap(),
                        )
                    }
                }
                "stop" => {
                    let server_process = {
                        let data_read = ctx.data.read().await;
                        data_read
                            .get::<ServerProcess>()
                            .expect("Expected ServerProcess in TypeMap.")
                            .clone()
                    };

                    {
                        let mut server_process = server_process.write().await;
                        commands::stop::run(
                            &command.data.options,
                            &mut server_process,
                            CONFIG.get("notify-id").unwrap(),
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

        let guild_id = GuildId(CONFIG.get::<u64>("guild-id").unwrap());

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::start::register(command))
                .create_application_command(|command| commands::stop::register(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:?}",
            commands
        );
    }
}

#[tokio::main]
async fn main() {
    let token = CONFIG.get::<String>("token").unwrap();

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        // Get a write lock to the data in the client
        // Use a block to make sure the lock is released
        let mut data = client.data.write().await;
        data.insert::<ServerProcess>(Arc::new(RwLock::new(None)));
    }

    if let Err(why) = client.start().await {
        println!("client error: {:?}", why);
    }
}
