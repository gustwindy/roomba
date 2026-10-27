mod roomba_gen;
mod commands;
mod utils;

use std::collections::HashMap;
use std::{str::FromStr};
use std::env;
use iso_currency::Currency;
use poise::serenity_prelude::{self as serenity, CacheHttp, CreateMessage, GetMessages};

use utils::{Data};
use serde::{Deserialize};

use crate::roomba_gen::{AskReplyRequest, HistoryMessage};
use crate::utils::{CurrencyRate, Error};

#[derive(Deserialize)]
struct RawCurrencyRate {
    quote: String,
    rate: f32
}

async fn event_handler(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    let _data = framework.user_data;
    let ctx = framework.serenity_context;

    match event {
        serenity::FullEvent::Message { new_message: message } => {
            let bot_id = ctx.cache.current_user().id.get();
            if message.author.id == bot_id {
                return Ok(());
            }
            if message.guild_id.is_none() || (message.guild_id.unwrap().get() != 1410959974842236930 && message.guild_id.unwrap().get() != 1073897040515452928) {
                return Ok(());
            }
            if message.content.to_ascii_lowercase().contains(&String::from("roomba")) || message.content.contains(&u64::to_string(&bot_id)) {
                let channel = message.channel_id;
                let mut messages = channel.messages(ctx.http(), GetMessages::new().limit(7).before(message.id.get()+1)).await?;
                let mut history: Vec<HistoryMessage> = vec![];
                let mut user_ids: HashMap<u64, String> = HashMap::new();

                messages.reverse();

                messages.iter().for_each(|msg| { // nobody told me there was a foreach in rust
                    history.push( HistoryMessage {
                        author_id: msg.author.id.get(),
                        id: msg.id.get(),
                        content: String::from(&msg.content)
                    });

                    user_ids.insert(msg.author.id.get(),String::from(msg.author.display_name()));
                });

                channel.broadcast_typing(ctx.http()).await?;

                let resp = roomba_gen::send_to_roomba(&env::var("URI")?, AskReplyRequest {
                    user_ids: user_ids,
                    bot_id: bot_id,
                    messages: history
                }).await?;
                let text = resp["content"].get("text");
                if text.is_some() {
                    let s = text.unwrap().as_str();
                    channel.send_message(ctx.http(),CreateMessage::new().content(s.unwrap_or("-# roomba sent nothing back."))).await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token: String = env::var("TOKEN").expect("so. there's no bot token (TOKEN)");

    let intents = serenity::GatewayIntents::all();

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

        println!("Did");
    }


    let data = Data {
        owner: u64::from_str(&env::var("OWNER").expect("so. there's no owner id (OWNER)")).expect("owner id not a number"),
        currencies: currencies.unwrap_or(HashMap::new())
    };

    // one day ill be able to figure out whats going on down here
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
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
