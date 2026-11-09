use std::{collections::HashMap, env};

use poise::{Command, CreateReply, serenity_prelude::{CreateEmbed}};

use crate::{roomba_gen::{self, AskReplyRequest, HistoryMessage}, utils::{Context, Data, Error}};

#[poise::command(slash_command, subcommands("ask"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn roomba(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong:3").await?;

    Ok(())
}

/// ask roomba something!
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn ask(ctx: Context<'_>, what: String) -> Result<(), Error> {
    let mut history: Vec<HistoryMessage> = vec![];
    let mut user_ids: HashMap<u64, String> = HashMap::new();
    let bot_id = ctx.cache().current_user().id.get();

    user_ids.insert(ctx.author().id.get(), String::from(ctx.author().display_name()));
    history.push(HistoryMessage {
        author_id: ctx.author().id.get(),
        id: ctx.id(),
        content: String::from(&what) // the frick
    });

    ctx.defer().await?;

    let resp = roomba_gen::send_to_roomba(&env::var("URI")?, AskReplyRequest {
        user_ids: user_ids,
        bot_id: bot_id,
        messages: history
    }).await?;

    let text = resp["content"].get("text");
    if text.is_some() {
        let s = text.unwrap().as_str().unwrap_or("-# roomba replied nothing");

        ctx.send(CreateReply::default().content(s).embed(CreateEmbed::default().title("asked roomba:").description(&what))).await?;
    } else {
        ctx.say("-# roomba replied nothing").await?;
    }

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        roomba()
    ]
}
