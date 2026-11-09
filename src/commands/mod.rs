use poise::Command;

use crate::utils::{Data, Error};
pub mod main_server;
pub mod user;


pub fn get() -> Vec<Command<Data, Error>>  {
    let mut commands: Vec<Command<Data, Error>> = vec![];

    commands.extend(user::get());
    commands.extend(main_server::get());

    return commands;
}
