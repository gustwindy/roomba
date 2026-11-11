use poise::{Command, CreateReply};

use crate::utils::{Context, Data, Error, discord_id_to_timestamp};

/// Checks the ping of roomba to the server.
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let msg = ctx.say("pinging...").await?;

    let actual = msg.to_owned().into_message().await?;

    let start = discord_id_to_timestamp(&ctx.id());
    let end = discord_id_to_timestamp(&actual.id.get());

    msg.edit(
        ctx,
        CreateReply::default().content(format!("pong :3 *({}ms)*", end - start)),
    )
    .await?;

    Ok(())
}
/*
/// (WIP) Converts a currency to another.
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel",
    check = "is_owner"
)]
async fn currency(
    ctx: Context<'_>,
    amt: u32,
    from: Option<String>,
    to: Option<String>,
) -> Result<(), Error> {
    ctx.say(format!(
        "unfinished command sorry {} {} {}",
        amt,
        from.unwrap_or(String::from("?")),
        to.unwrap_or(String::from("?"))
    ))
    .await?;

    Ok(())
} */

pub fn register() -> Vec<Command<Data, Error>> {
    vec![ping()]
}
