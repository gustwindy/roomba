use crate::utils::{Context, Error};

#[poise::command(slash_command, subcommands("string_theory", "clover", "so_sorry"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
pub async fn song(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn string_theory(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[string theory](https://www.youtube.com/watch?v=84UfQLzYWws)").await?;

    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn clover(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[clover](https://www.youtube.com/watch?v=mFcG1XVHjzk)").await?;

    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn so_sorry(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[so sorry !!](https://www.youtube.com/watch?v=qdoFhnIG98g)").await?;

    Ok(())
}
