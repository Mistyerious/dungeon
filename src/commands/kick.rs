use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use serenity::prelude::*;
use serenity::model::channel::Message;

#[command]
pub async fn kick(ctx: &Context, msg: &Message) -> CommandResult {

    msg.reply(ctx, "IDK WHAT TO DO").await?;

    Ok(())
}