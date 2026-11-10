use poise::{
    Command,
    serenity_prelude::{CreateMessage, MessageId},
};
use std::str::FromStr;

use crate::utils::{Context, Data, Error, is_owner};

#[poise::command(
    slash_command,
    subcommands("say", "send_self_reply", "reply", "say_antilog"),
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn i(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong:3").await?;

    Ok(())
}

/// [instance owner only]
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    check = "is_owner"
)]
async fn say(ctx: Context<'_>, msg: String) -> Result<(), Error> {
    ctx.say(msg).await?;

    Ok(())
}

/// [instance owner only]
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    check = "is_owner"
)]
async fn send_self_reply(ctx: Context<'_>, content: String) -> Result<(), Error> {
    let msg = ctx.say("_ _").await?;
    msg.delete(ctx).await?;
    ctx.say(content).await?;

    Ok(())
}

/// [instance owner only]
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|PrivateChannel",
    check = "is_owner"
)]
async fn reply(ctx: Context<'_>, to_reply_to: String, content: String) -> Result<(), Error> {
    let channel = ctx.guild_channel().await;
    if channel.is_none() {
        ctx.say("something is very wrong").await?;

        return Ok(());
    }
    channel
        .unwrap()
        .message(ctx.http(), MessageId::new(u64::from_str(&to_reply_to)?))
        .await?
        .reply_ping(ctx.http(), content)
        .await?;

    Ok(())
}

/// [instance owner only]
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|PrivateChannel",
    check = "is_owner"
)]
async fn say_antilog(ctx: Context<'_>, msg: String) -> Result<(), Error> {
    let channel = ctx.guild_channel().await;
    if channel.is_none() {
        ctx.say("something is very wrong").await?;

        return Ok(());
    }
    channel
        .unwrap()
        .send_message(ctx.http(), CreateMessage::default().content(msg))
        .await?;

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![i()]
}
