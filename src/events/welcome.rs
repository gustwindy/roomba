use std::{sync::Arc, time::Duration};

use poise::serenity_prelude::{
    self, CacheHttp, ChannelId, Color, CreateEmbed, CreateMessage, Http, Mentionable, Message,
    MessageReference, MessageReferenceKind, MessageType, ReactionType, RoleId,
};

use crate::utils::{Data, Error, config::Config};

async fn handle_message(arc: Arc<Http>, config: &Config, message: &Message) -> Result<(), Error> {
    if !config.host_server.join_logs.eq(&message.channel_id.get())
        || !message.kind.eq(&MessageType::MemberJoin)
        || message.reactions.iter().any(|r| r.me)
    {
        return Ok(());
    };

    let http = arc.clone();

    let user = &message.author;
    let welcome = ChannelId::from(config.host_server.welcome);

    let embed = CreateEmbed::new()
        .title("welcome to windy's warehouse!")
        .description(format!(
            "-# Account Creation: <t:{}:R>",
            user.created_at().unix_timestamp()
        ))
        .image(
            user.avatar_url()
                .unwrap_or(String::from("https://example.com")),
        )
        .color(Color::from_rgb(182, 138, 249));

    welcome
        .send_message(
            &http,
            CreateMessage::new()
                .content(user.mention().to_string())
                .embed(embed),
        )
        .await?;

    message
        .react(&http, ReactionType::Unicode(String::from("✅")))
        .await?;

    let verify_msg = &http
        .get_message(
            config.host_server.verify_channel.into(),
            config.host_server.verify_msg.into(),
        )
        .await?;

    user.dm(
        &http,
        CreateMessage::new().reference_message(
            MessageReference::new(MessageReferenceKind::Forward, verify_msg.channel_id)
                .message_id(verify_msg.id),
        ),
    )
    .await?;

    let member = message.member(&http).await?;
    let role = config
        .host_server
        .roles
        .first()
        .expect("thats literally not even possible")
        .to_owned();
    let guild = config.host_server.guild_id;
    let http = arc.clone();

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(20 * 60)).await;

        if member
            .user
            .has_role(&http, guild, RoleId::new(role))
            .await
            .unwrap_or(true)
        {
            return;
        }

        member.kick_with_reason(&http, "took too long").await.ok();
    });

    Ok(())
}

pub async fn event(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity_prelude::FullEvent,
) -> Result<(), Error> {
    let ctx = framework.serenity_context;
    let data = &framework.user_data.config;
    match event {
        serenity_prelude::FullEvent::Message {
            new_message: message,
        } => return handle_message(ctx.http.clone(), data, message).await,

        serenity_prelude::FullEvent::ReactionAdd { add_reaction } => {
            if data
                .host_server
                .verify_channel
                .eq(&add_reaction.channel_id.get())
            {
                if let Some(member) = &add_reaction.member {
                    member
                        .kick_with_reason(ctx.http(), "clicked on checkmark")
                        .await?;
                }
            }
        }

        serenity_prelude::FullEvent::Ready {
            data_about_bot: _ready,
        } => {
            let messages = ctx
                .http
                .get_messages(ChannelId::from(data.host_server.join_logs), None, Some(25))
                .await?;

            for msg in messages.iter() {
                handle_message(ctx.http.clone(), data, msg).await?;
            }
        }
        _ => {}
    }
    Ok(())
}
