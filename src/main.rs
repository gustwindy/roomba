mod commands;
mod utils;

use std::collections::HashMap;
use std::{str::FromStr};
use std::env;
use iso_currency::Currency;
use poise::serenity_prelude::{self as serenity};

use utils::{Data};
use serde::{Deserialize};

use crate::utils::CurrencyRate;

#[derive(Deserialize)]
struct RawCurrencyRate {
    quote: String,
    rate: f32
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token: String = env::var("TOKEN").expect("so. there's no bot token (TOKEN)");

    let intents = serenity::GatewayIntents::non_privileged();

    let mut currencies: Option<HashMap<String, CurrencyRate>> = None;
    let commands = commands::get();

    let resp = reqwest::get("https://api.frankfurter.dev/v2/rates?base=usd").await;
    if resp.is_ok() {
        let raw: Vec<RawCurrencyRate> = serde_json::from_str(&resp.unwrap().text().await.expect("failed to parse currencies ?! (1)")).expect("failed to parse currencies ?! (2)");

        currencies = Some(raw.iter().map(|f| {
            let data = Currency::from_code(&f.quote).unwrap_or(Currency::from_code("USD").expect("what the hell"));

            (String::from(&f.quote), CurrencyRate {
                short: String::from(&f.quote),

                symbol: data.symbol().to_string(),
                full: String::from(data.name()),

                from_currency: String::from("USD"),
                from_amt: 1.0,
                to: f.rate
            })
        }).collect());

        println!("Could");
    }


    let data = Data {
        owner: u64::from_str(&env::var("OWNER").expect("so. there's no owner id (OWNER)")).expect("owner id not a number"),
        currencies: currencies.unwrap_or(HashMap::new())
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
