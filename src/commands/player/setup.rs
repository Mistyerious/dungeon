use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;

pub async fn setup(cmd: ApplicationCommandInteraction, ctx: Context, _interaction: Interaction) {
    cmd.create_interaction_response(ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| msg.embed(|e| {
                e.title("This works!")
                    .description("THIS WORKS WELL!")
            }))
    }).await.unwrap();
}