mod commands;
mod events;
mod roomba_gen;
mod utils;

use poise::serenity_prelude::{self as serenity, GuildId};
use std::collections::HashMap;
use std::env;

use serde::Deserialize;
use tokio::fs;
use utils::Data;

use crate::utils::config::Config;
use crate::utils::{Error, currencies};

#[derive(Deserialize)]
struct RawCurrencyRate {
    quote: String,
    rate: f32,
}

async fn event_handler(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    events::dispatch(framework, event).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token: String = env::var("TOKEN").expect("so. there's no bot token (TOKEN)");
    let preset: String = env::var("PRESET").expect("so. there's no config preset (PRESET)");

    let intents = serenity::GatewayIntents::all();

    let user_commands = commands::user::get();
    let server_commands = commands::main_server::get();

    let config: Config = serde_json::from_str(
        &fs::read_to_string(format!("include/config/{}.json", preset))
            .await
            .expect("no preset config with that name"),
    )
    .expect("invalid config");
    let data = Data {
        currencies: currencies::fetch_currencies()
            .await
            .unwrap_or(HashMap::new()),
        config: config,
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get(),
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, ready, _framework| {
            println!("logged in as: {}", ready.user.name);

            Box::pin(async move {
                poise::builtins::register_globally(ctx, &user_commands).await?;
                poise::builtins::register_in_guild(
                    ctx,
                    &server_commands,
                    GuildId::new(data.config.host_server.guild_id),
                )
                .await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
