use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use tokio::sync::Mutex;

use crate::utils::booru::Listing;
pub mod booru;
pub mod config;
pub mod currencies;
pub mod images;

pub struct Data {
    pub currencies: HashMap<String, CurrencyRate>,
    pub config: config::Config,

    pub stored_yuri: Mutex<Vec<Listing>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct CurrencyRate {
    pub short: String,

    pub symbol: String,
    pub full: String,

    pub from_currency: String,
    pub from_amt: f32,
    pub to: f32,
}

pub fn time(id: &u64) -> u64 {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_millis(0));

    return (now.as_millis() as u64) - discord_id_to_timestamp(&id);
}

pub async fn is_owner(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx.author().id.get() != ctx.data().config.owner {
        ctx.say("https://cdn.discordapp.com/attachments/1234467030011351052/1563281568989188238/gustwindy_roomba.gif").await?;

        return Ok(false);
    }
    Ok(true)
}

const DISCORD_START_DATE: u64 = 1_420_070_400_000;

pub fn discord_id_to_timestamp(snowflake: &u64) -> u64 {
    (snowflake >> 22) + DISCORD_START_DATE
}
