use poise::Command;

use crate::utils::{Data, Error};
mod random;
mod text_conversion;

pub fn get() -> Vec<Command<Data, Error>>  {
    let mut commands: Vec<Command<Data, Error>> = vec![];

    commands.extend(random::register());
    commands.extend(text_conversion::register());

    return commands;
}
