use crate::utils::{Context, Error};

#[poise::command(slash_command, subcommands("string_theory", "clover", "so_sorry", "the_rest"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
pub async fn song(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// so whooo is sheeee who siits on ivory
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn string_theory(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[string theory](https://www.youtube.com/watch?v=84UfQLzYWws)").await?;

    Ok(())
}

/// the blinding white of freshly mounted linen
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn clover(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[clover](https://www.youtube.com/watch?v=mFcG1XVHjzk)").await?;

    Ok(())
}

/// i knoww that its just a feeling disappearing
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn so_sorry(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[so sorry !!](https://www.youtube.com/watch?v=qdoFhnIG98g)").await?;

    Ok(())
}

/// the rest
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn the_rest(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("[the rest](https://www.youtube.com/watch?v=AG29hTaJC3U)").await?;

    Ok(())
}
