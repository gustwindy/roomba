use poise::Command;

use crate::utils::{Data, Error};

pub mod random;

pub fn get() -> Vec<Command<Data, Error>>  {
    let mut commands: Vec<Command<Data, Error>> = vec![];

    commands.extend(random::register());

    return commands;
}
