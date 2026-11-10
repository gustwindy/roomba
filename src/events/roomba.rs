use std::{collections::HashMap, env};

use poise::serenity_prelude::{self, CacheHttp, CreateMessage, GetMessages};

use crate::{
    roomba_gen::{self, AskReplyRequest, HistoryMessage},
    utils::{Data, Error},
};

pub async fn event(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity_prelude::FullEvent,
) -> Result<(), Error> {
    let ctx = framework.serenity_context;
    let data = &framework.user_data.config;
    match event {
        serenity_prelude::FullEvent::Message {
            new_message: message,
        } => {
            let bot_id = ctx.cache.current_user().id.get();
            if message.author.id == bot_id {
                return Ok(());
            }

            if message.guild_id.is_some()
                && (!data
                    .roomba_allowed_servers
                    .contains(&message.guild_id.unwrap().get()))
            {
                return Ok(());
            }

            if message
                .content
                .to_ascii_lowercase()
                .contains(&String::from("roomba"))
                || message.content.contains(&u64::to_string(&bot_id))
            {
                let channel = message.channel_id;
                let mut messages = channel
                    .messages(
                        ctx.http(),
                        GetMessages::new().limit(7).before(message.id.get() + 1),
                    )
                    .await?;
                let mut history: Vec<HistoryMessage> = vec![];
                let mut user_ids: HashMap<u64, String> = HashMap::new();

                messages.reverse();

                messages.iter().for_each(|msg| {
                    // nobody told me there was a foreach in rust
                    history.push(HistoryMessage {
                        author_id: msg.author.id.get(),
                        id: msg.id.get(),
                        content: String::from(&msg.content),
                    });

                    user_ids.insert(msg.author.id.get(), String::from(msg.author.display_name()));
                });

                channel.broadcast_typing(ctx.http()).await?;

                let resp = roomba_gen::send_to_roomba(
                    &env::var("URI")?,
                    AskReplyRequest {
                        user_ids: user_ids,
                        bot_id: bot_id,
                        messages: history,
                    },
                )
                .await?;
                let text = resp["content"].get("text");
                if text.is_some() {
                    let s = text.unwrap().as_str();
                    channel
                        .send_message(
                            ctx.http(),
                            CreateMessage::new()
                                .content(s.unwrap_or("-# roomba sent nothing back.")),
                        )
                        .await?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
