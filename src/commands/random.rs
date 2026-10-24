use std::str::FromStr;
use poise::{Command, serenity_prelude::{CreateMessage, MessageId}};

use crate::utils::{Context, Data, Error, is_owner};

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
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

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|PrivateChannel", check="is_owner")]
async fn reply(ctx: Context<'_>, to_reply_to: String, content: String) -> Result<(), Error> {
    let channel = ctx.guild_channel().await;
    if channel.is_none() {
        ctx.say("something is very wrong").await?;

        return Ok(());
    }
    channel.unwrap().message(ctx.http(), MessageId::new(u64::from_str(&to_reply_to)?)).await?.reply_ping(ctx.http(), content).await?;

    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|PrivateChannel", check="is_owner")]
async fn say_antilog(ctx: Context<'_>, msg: String) -> Result<(), Error> {
    let channel = ctx.guild_channel().await;
    if channel.is_none() {
        ctx.say("something is very wrong").await?;

        return Ok(());
    }
    channel.unwrap().send_message(ctx.http(), CreateMessage::default().content(msg)).await?;

    Ok(())
}


#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn currency(ctx: Context<'_>, amt: u32, from: Option<String>, to: Option<String>) -> Result<(), Error> {
    println!();

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        ping(), reply(), say(), say_antilog(), send_self_reply()
    ]
}
