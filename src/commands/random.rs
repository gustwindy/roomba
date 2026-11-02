use std::time;

use poise::{Command, CreateReply};
use tokio::time::Instant;

use crate::utils::{Context, Data, Error, is_owner};

/// Checks the ping of roomba to the server.
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {

    let now = Instant::now();
    let msg = ctx.say("pinging...").await?;
    msg.edit(ctx, CreateReply::default().content(format!("pong :3 *({}ms)*", now.elapsed().as_millis()))).await?;


    Ok(())
}

/// (WIP) Converts a currency to another.
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn currency(ctx: Context<'_>, amt: u32, from: Option<String>, to: Option<String>) -> Result<(), Error> {
    ctx.say(format!("unfinished command sorry {} {} {}", amt,from.unwrap_or(String::from("?")),to.unwrap_or(String::from("?")))).await?;

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        ping()
    ]
}
