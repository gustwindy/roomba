mod commands;
mod utils;

use std::{str::FromStr};
use std::env;
use poise::serenity_prelude::{self as serenity};

use utils::{Data};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token: String = env::var("TOKEN").expect("so. there's no bot token (TOKEN)");

    let intents = serenity::GatewayIntents::non_privileged();

    let commands = commands::get();

    let data = Data {
        owner: u64::from_str(&env::var("OWNER").expect("so. there's no owner id (OWNER)")).expect("owner id not a number")
    };

    // one day ill be able to figure out whats going on down here
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            println!("logged in as: {}", ready.user.name);
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
