mod commands;
mod handlers;
mod services;
mod utils;

use std::env;

use dotenv::dotenv;
use serenity::{
    async_trait,
    model::prelude::{command::Command, Interaction, InteractionResponseType, Ready},
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "rank" => commands::rank::run(&command.data.options).await,
                "player_info" => commands::player_info::run(&command.data.options).await,
                "assistant_manager_status" => {
                    commands::assistant_manager_status::run(&command.data.options).await
                }
                "staff_management" => commands::staff_management::run(&command.data.options).await,
                "manager_status" => commands::manager_status::run(&command.data.options).await,
                "team_info" => commands::team_info::run(&command.data.options).await,
                _ => (Some("not implemented".to_string()), None),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .content(content.0.unwrap_or("".to_string()))
                                .set_embeds(content.1)
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }

    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands_list = vec![
            commands::ping::register,
            commands::rank::register,
            commands::player_info::register,
            commands::manager_status::register,
            commands::assistant_manager_status::register,
            commands::staff_management::register,
            commands::team_info::register,
        ];

        for register in commands_list {
            let _ =
                Command::create_global_application_command(&ctx.http, |command| register(command))
                    .await;
        }

        println!("All slash commands loaded!");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("Expected a bot token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
