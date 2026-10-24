use poise::Command;

use crate::utils::{Context, Data, Error};

mod songs;
use songs::song;

#[poise::command(slash_command, subcommands("song"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn fridge(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}


pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        fridge()
    ]
}
