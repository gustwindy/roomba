use poise::{Command, CreateReply, serenity_prelude::RoleId};

use crate::utils::{Context, Data, Error};

/// verifies you
#[poise::command(slash_command, install_context = "Guild")]
async fn verify(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    if !data
        .config
        .host_server
        .verify_channel
        .eq(&ctx.channel_id().get())
    {
        ctx.send(
            CreateReply::new()
                .ephemeral(true)
                .content("you're not supposed to do that in here"),
        )
        .await?;
        return Ok(());
    };
    let user = ctx.author();
    if user
        .has_role(
            ctx,
            data.config.host_server.guild_id,
            RoleId::new(
                data.config
                    .host_server
                    .roles
                    .first()
                    .expect("frick you")
                    .to_owned(),
            ),
        )
        .await?
    {
        ctx.send(CreateReply::new().ephemeral(true).content("what the frick"))
            .await?;
        return Ok(());
    }

    let member = ctx.author_member().await.unwrap();

    // new things everyday
    let roles: Vec<RoleId> = data
        .config
        .host_server
        .roles
        .iter()
        .map(|r| RoleId::new(*r))
        .collect();
    member.add_roles(ctx, &roles).await?;

    ctx.send(
        CreateReply::new()
            .ephemeral(true)
            .content("thank you for verifying, you may now chat !!"),
    )
    .await?;
    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![verify()]
}
