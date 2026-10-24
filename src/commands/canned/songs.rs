use crate::utils::{Context, Error};

#[poise::command(slash_command, subcommands("do_you_know"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
pub async fn song(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn do_you_know(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[do you know?](https://youtu.be/EHBFKhLUVig?t=109)").await?;

    Ok(())
}
