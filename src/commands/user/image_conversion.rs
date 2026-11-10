use libvips::{VipsImage, ops};
use poise::{
    Command, CreateReply,
    serenity_prelude::{Attachment, CreateAttachment, User},
};

use crate::utils::{
    Context, Data, Error,
    images::{linear_without_the_fucking_alpha, mask_image, overlay},
};

#[poise::command(
    slash_command,
    subcommands("lesbian", "gay", "straight", "bi"),
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn image(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

pub fn generate(flavor: &str, original: &VipsImage) -> Result<VipsImage, Error> {
    let mut color = VipsImage::new_from_file(&format!("include/ring/{flavor}.png"))?;

    let mask_bg = VipsImage::new_from_file("include/ring/mask/bg.png")?;
    let mask_dark = VipsImage::new_from_file("include/ring/mask/darkened.png")?;
    let mask_inner = VipsImage::new_from_file("include/ring/mask/inner.png")?;
    let mask_outer = VipsImage::new_from_file("include/ring/mask/outer.png")?;

    let width = mask_bg.get_width();
    let height = mask_bg.get_height();

    // im gonna kill you
    let resized = ops::resize_with_opts(
        original,
        ((width - 140) as f64 / original.get_width() as f64) as f64,
        &ops::ResizeOptions {
            vscale: ((height - 140) as f64 / original.get_height() as f64) as f64,
            kernel: ops::Kernel::Cubic,
            ..Default::default()
        },
    )?;

    let padded = ops::embed(&resized, 70, 70, width, height)?;

    let mut cut_out = mask_image(&padded, &mask_inner)?;
    let ring_outer = mask_image(&color, &mask_outer)?;

    color = linear_without_the_fucking_alpha(&color, &mut [1.0], &mut [-50.0])?;

    let ring_bg = mask_image(&color, &mask_bg)?;
    let ring_dark = mask_image(&color, &mask_dark)?;

    cut_out = overlay(&cut_out, &ring_bg)?;
    cut_out = overlay(&cut_out, &ring_outer)?;
    cut_out = overlay(&cut_out, &ring_dark)?;

    Ok(cut_out)
}

async fn lazy_pfp(
    flavor: &str,
    ctx: Context<'_>,
    user: Option<User>,
    file: Option<Attachment>,
) -> Result<(), Error> {
    // wow i made this even more unreadable
    let input = if let Some(user) = user {
        reqwest::get(user.face()).await?.bytes().await?.to_vec()
    } else if let Some(file) = file {
        file.download().await?
    } else {
        ctx.say("you did not specify a file or a user").await?;
        return Ok(());
    };

    ctx.defer().await?;

    let flavor = flavor.to_owned();

    let output = tokio::task::spawn_blocking(move || {
        let img = VipsImage::new_from_buffer(&input, "")?;
        let img = generate(&flavor, &img)?;

        Ok::<Vec<u8>, Error>(img.image_write_to_buffer(".png")?)
    })
    .await??;

    ctx.send(
        CreateReply::default().attachment(CreateAttachment::bytes(output, "read_if_cute.png")),
    )
    .await?;

    Ok(())
}

/// adds a lesbian ring around an image/someone's pfp
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn lesbian(
    ctx: Context<'_>,
    user: Option<User>,
    file: Option<Attachment>,
) -> Result<(), Error> {
    lazy_pfp("lesbian", ctx, user, file).await?;
    Ok(())
}

/// adds a gay (mlm) ring around an image/someone's pfp
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn gay(ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
    lazy_pfp("gay", ctx, user, file).await?;
    Ok(())
}

/// adds a straight flag ring around an image/someone's pfp
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn straight(
    ctx: Context<'_>,
    user: Option<User>,
    file: Option<Attachment>,
) -> Result<(), Error> {
    lazy_pfp("straight", ctx, user, file).await?;
    Ok(())
}

/// adds a bi ring around an image/someone's pfp
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn bi(ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
    lazy_pfp("bi", ctx, user, file).await?;
    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![image()]
}
