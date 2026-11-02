use poise::{Command};

use crate::utils::{Context, Data, Error, is_owner};

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong:3").await?;

    Ok(())
}

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
