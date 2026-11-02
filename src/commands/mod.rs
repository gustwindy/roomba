use poise::Command;

use crate::utils::{Data, Error};
mod random;
mod text_conversion;
mod image_conversion;
mod canned;
mod owner;

pub fn get() -> Vec<Command<Data, Error>>  {
    let mut commands: Vec<Command<Data, Error>> = vec![];

    commands.extend(random::register());
    commands.extend(text_conversion::register());
    commands.extend(image_conversion::register());
    commands.extend(canned::register());
    commands.extend(owner::register());

    return commands;
}
