use poise::Command;

use crate::utils::{Data, Error};
mod canned;
mod find;
mod image_conversion;
mod owner;
mod random;
mod roomba;
mod technical;
mod text_conversion;

pub fn get() -> Vec<Command<Data, Error>> {
    let mut commands: Vec<Command<Data, Error>> = vec![];

    commands.extend(canned::register());
    commands.extend(find::register());
    commands.extend(image_conversion::register());
    commands.extend(owner::register());
    commands.extend(random::register());
    commands.extend(roomba::register());
    commands.extend(technical::register());
    commands.extend(text_conversion::register());

    return commands;
}
