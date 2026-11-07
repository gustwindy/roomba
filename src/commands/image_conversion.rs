use photon_rs::{PhotonImage, Rgba, effects, multiple, native::{open_image, open_image_from_bytes}, transform::{SamplingFilter, padding_bottom, padding_left, padding_right, padding_top, resize}};
use poise::{Command, CreateReply, serenity_prelude::{Attachment, CreateAttachment, User}};

use crate::utils::{Context, Data, Error};

#[poise::command(slash_command, subcommands("lesbian", "gay", "straight", "bi"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn image(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
fn mask_image(original: &PhotonImage, mask: &PhotonImage) -> PhotonImage {
    let mut img = mask.clone();

    multiple::blend(&mut img, &original, "atop");

    return img;
}

fn generate(flavor: &str, original: &PhotonImage) -> Result<PhotonImage, Box<dyn std::error::Error>> { // flavor wth // like foley
    let mut color = open_image(format!("include/ring/{}.png",flavor))?;
    let img = original.clone();

    let mask_bg = open_image("include/ring/mask/bg.png")?;
    let mask_dark = open_image("include/ring/mask/darkened.png")?;
    let mask_inner = open_image("include/ring/mask/inner.png")?;
    let mask_outer = open_image("include/ring/mask/outer.png")?;

    let resized = resize(&img, mask_bg.get_width()-140, mask_bg.get_height()-140, SamplingFilter::CatmullRom);
    let padded = padding_bottom(&padding_left(&padding_right(&padding_top(&resized, 70, Rgba::new(0, 0, 0, 0)), 70, Rgba::new(0, 0, 0, 0)), 70, Rgba::new(0, 0, 0, 0)), 70, Rgba::new(0, 0, 0, 0));

    let mut cut_out = mask_image(&padded, &mask_inner);
    let ring_outer = mask_image(&color, &mask_outer);

    effects::adjust_brightness(&mut color, -50);

    let ring_bg = mask_image(&color, &mask_bg);
    let ring_dark = mask_image(&color, &mask_dark);

    multiple::watermark(&mut cut_out, &ring_bg, 0, 0);
    multiple::watermark(&mut cut_out, &ring_outer, 0, 0);
    multiple::watermark(&mut cut_out, &ring_dark, 0, 0);

    Ok(cut_out)
}

async fn lazy_pfp(flavor: &str, ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
    let img: PhotonImage;

    if user.is_some() {
        img = open_image_from_bytes(&reqwest::Client::new().get(user.expect("checked earlier").face()).send().await?.bytes().await?.to_vec())?;
    } else if file.is_some() {
        img = PhotonImage::new_from_byteslice(file.unwrap().download().await?);
    } else {
        ctx.say("you did not specify a file or a user").await?;

        return Ok(());
    }

    ctx.defer().await?;
    let img = generate(flavor, &img).expect("frick you");

    ctx.send(CreateReply::default()
        .attachment(CreateAttachment::bytes(img.get_bytes(), "read_if_cute.png"))).await?;

    Ok(())
}

/// adds a lesbian ring around an image/someone's pfp
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn lesbian(ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
    lazy_pfp("lesbian", ctx, user, file).await?;
    Ok(())
}

/// adds a gay (mlm) ring around an image/someone's pfp
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn gay(ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
    lazy_pfp("gay", ctx, user, file).await?;
    Ok(())
}

/// adds a straight flag ring around an image/someone's pfp
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn straight(ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
    lazy_pfp("straight", ctx, user, file).await?;
    Ok(())
}

/// adds a bi ring around an image/someone's pfp
#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel")]
async fn bi(ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
    lazy_pfp("bi", ctx, user, file).await?;
    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        image()
    ]
}
