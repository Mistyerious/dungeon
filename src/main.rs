use std::env;
use dotenv::dotenv;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::model::id::GuildId;

mod commands;
use crate::commands::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);


        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an int")
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("setup").description("Setup ig")
                })
        })
            .await;

        println!("I now have the following guild slash commands: {:#?}", commands)
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(ref command) = interaction {
            match command.data.name.as_str() {
                "setup" => setup(command.clone(), ctx, interaction).await,
                _ => println!("Not implemented")
            }
        }
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why)
    }
}