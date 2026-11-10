use poise::Command;

use crate::utils::{Data, Error};
mod canned;
mod image_conversion;
mod owner;
mod random;
mod roomba;
mod text_conversion;

pub fn get() -> Vec<Command<Data, Error>> {
    let mut commands: Vec<Command<Data, Error>> = vec![];

    commands.extend(random::register());
    commands.extend(text_conversion::register());
    commands.extend(image_conversion::register());
    commands.extend(canned::register());
    commands.extend(owner::register());
    commands.extend(roomba::register());

    return commands;
}
