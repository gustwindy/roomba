use poise::Command;

use crate::utils::{Context, Data, Error, is_owner};

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong:3").await?;

    Ok(())
}
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn say(ctx: Context<'_>, msg: String) -> Result<(), Error> {
    ctx.say(msg).await?;

    Ok(())
}
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn send_self_reply(ctx: Context<'_>, content: String) -> Result<(), Error> {
    let msg = ctx.say("_ _").await?;
    msg.delete(ctx).await?;
    ctx.say(content).await?;

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        ping(), say(), send_self_reply()
    ]
}
