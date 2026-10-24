use std::collections::HashMap;

pub struct Data {
    pub owner: u64,
    pub currencies: HashMap<String, CurrencyRate>
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


pub async fn is_owner(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx.author().id.get() != ctx.data().owner {
        ctx.say("https://cdn.discordapp.com/attachments/1234467030011351052/1563281568989188238/gustwindy_roomba.gif").await?;

        return Ok(false);
    }
    Ok(true)
}
