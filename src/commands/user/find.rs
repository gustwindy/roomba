use poise::{
    Command, CreateReply,
    serenity_prelude::{CreateEmbed, CreateEmbedFooter},
};

use crate::utils::{
    Context, Data, Error,
    booru::{Listing, get_random_images},
    time,
};

#[poise::command(
    slash_command,
    subcommands("yuri"),
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn find(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}
/* frick your credit
fn generate_credit_thing(source: String) -> Result<String, Error> {
    let supported = RegexSet::new(&[
        r"https?://(www\.)deviantart\.com/(\w+)/art",
        r"https?://\w*(x|twitter)\.(com|org)/(\w+)/status",
        r"https?://[[\w+.][^/]]+",
    ])?;
    let data = &[(1, "@!"), (2, "@! on twitter"), (0, "some guy on !")];

}

*/

fn booru_embed(post: Listing, delay: u64) -> CreateReply {
    return CreateReply::new().embed(
        CreateEmbed::new()
            .title("source")
            .url(post.source)
            .image(post.file_url)
            .footer(CreateEmbedFooter::new(format!("{}ms", delay))),
    );
}

/// Finds you yuri.
#[poise::command(
    slash_command,
    install_context = "User",
    interaction_context = "Guild|BotDm|PrivateChannel"
)]
async fn yuri(ctx: Context<'_>) -> Result<(), Error> {
    let config = &ctx.data().config.commands.find;

    let mut all = ctx.data().stored_yuri.lock().await;
    let mut sent = false;

    if all.len() > 1 {
        sent = true;
        ctx.send(booru_embed(all.pop().expect("there's 2"), time(&ctx.id())))
            .await?;
    }

    if (all.len() as f32) < ((config.limit as f32) * config.refetch_ratio) {
        let images = get_random_images(&config.yuri_tags.to_owned(), &config.limit).await?;
        all.extend(images);
        println!("refetching")
    }

    if all.len() > 1 && !sent {
        ctx.send(booru_embed(
            all.pop().expect("we populated it"),
            time(&ctx.id()),
        ))
        .await?;
    }

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![find()]
}
