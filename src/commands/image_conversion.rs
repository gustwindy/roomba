use photon_rs::{PhotonImage, Rgba, multiple, native::{open_image, open_image_from_bytes}, transform::{SamplingFilter, padding_bottom, padding_left, padding_right, padding_top, resize}};
use poise::{Command, CreateReply, serenity_prelude::{Attachment, CreateAttachment, User}};

use crate::utils::{Context, Data, Error, is_owner};

#[poise::command(slash_command, subcommands("lesbian"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn image(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn lesbian(ctx: Context<'_>, user: Option<User>, file: Option<Attachment>) -> Result<(), Error> {
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

    let mut mask = open_image("include/ring/mask.png").expect("do we not have include");

    let ring = open_image("include/ring/lesbian.png").expect("do we not have include");
    let resized = resize(&img, mask.get_width()-140, mask.get_height()-140, SamplingFilter::CatmullRom);
    let padded = padding_bottom(&padding_left(&padding_right(&padding_top(&resized, 70, Rgba::new(0, 0, 0, 0)), 70, Rgba::new(0, 0, 0, 0)), 70, Rgba::new(0, 0, 0, 0)), 70, Rgba::new(0, 0, 0, 0));
    // what the heck is that

    multiple::blend(&mut mask, &padded, "atop");
    multiple::watermark(&mut mask, &ring, 0, 0);


    ctx.send(CreateReply::default()
        .attachment(CreateAttachment::bytes(mask.get_bytes(), "image.png"))).await?;

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        image()
    ]
}
