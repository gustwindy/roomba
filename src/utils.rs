pub struct Data {
    pub owner: u64
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn is_owner(ctx: Context<'_>) -> Result<bool, Error> {
    if ctx.author().id.get() != ctx.data().owner {
        ctx.say("https://cdn.discordapp.com/attachments/1465548194065682474/1537852379658059796/ughh_i_love_sabi_somuchhhhh.gif").await?;

        return Ok(false);
    }
    Ok(true)
}
