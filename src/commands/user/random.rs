use poise::{
    Command, CreateReply,
    serenity_prelude::{CreateEmbed, Mentionable, User},
};
use std::str::FromStr;

use crate::utils::{Context, Data, Error, discord_id_to_timestamp, is_owner};

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

fn copyable(data: &[(&str, String)]) -> String {
    let mut lines: Vec<String> = vec![];

    data.iter().for_each(|d| {
        lines.push(format!(
            "-# {}\n> `{}`",
            String::from(d.0),
            String::from(&d.1)
        ));
    });

    return lines.join("\n");
}

fn displayable(data: &[(&str, String)]) -> CreateEmbed {
    return CreateEmbed::new().description(&copyable(data));
}

/// Shows potentially useless data on a user.
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn get_user_data(ctx: Context<'_>, user: User) -> Result<(), Error> {
    ctx.send(
        CreateReply::new()
            .ephemeral(true)
            .content(format!("## {}", user.mention().to_string()))
            .embed(displayable(&[
                ("Display Name", String::from(user.display_name())),
                ("User Id", user.id.to_string()),
                ("Icon URL", user.face()),
                (
                    "Server Tag",
                    user.primary_guild
                        .unwrap_or_default()
                        .tag
                        .unwrap_or(String::from("no tag")),
                ),
            ])),
    )
    .await?;

    Ok(())
}

/// Shows everything about a any discord ID that is possible.
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn get_id_data(ctx: Context<'_>, id: String) -> Result<(), Error> {
    let num = u64::from_str(&id)?;
    let stamp = discord_id_to_timestamp(&num);
    ctx.send(
        CreateReply::new()
            .ephemeral(true)
            .content(format!("## {}", format!("id at <t:{}>", &stamp / 1000)))
            .embed(displayable(&[("Unix Timestamp", stamp.to_string())])),
    )
    .await?;

    Ok(())
}

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
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![ping(), get_id_data(), get_user_data()]
}
