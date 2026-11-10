use poise::Command;

mod verification;

use crate::utils::{Data, Error};
pub fn get() -> Vec<Command<Data, Error>> {
    let mut commands: Vec<Command<Data, Error>> = vec![];

    commands.extend(verification::register());

    return commands;
}
