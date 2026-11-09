use poise::Command;

use crate::utils::{Context, Data, Error};

mod songs;
mod lyrics;
use songs::song;
use lyrics::lyric;

#[poise::command(slash_command, subcommands("song","lyric"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn fridge(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}


pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        fridge()
    ]
}
