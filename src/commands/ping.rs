use serenity::{framework::standard::{macros::command, CommandResult}, prelude::Context, model::prelude::Message};

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}