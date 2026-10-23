use poise::{Command, CreateReply};

use crate::utils::{Context, Data, Error, is_owner};

#[poise::command(slash_command, subcommands("serif", "monospace"), install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn text(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fn get_alphabet() -> String {
    String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
}

fn convert_to(text: &str, to: &str) -> String {
    let alphabet = get_alphabet();

    let conv: Vec<char> = text.chars().map(|c| {
        let num = alphabet.find(c);

        if num.is_some() {
            return to.chars().nth(num.expect("just checked")).expect("that's not possible");
        }

        c
    }).collect();

    conv.iter().collect()
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn serif(ctx: Context<'_>, content: String) -> Result<(), Error> {
    ctx.send(CreateReply::default()
        .ephemeral(true)
        .content(format!("```\n{}\n```", convert_to(&content, "𝖠𝖡𝖢𝖣𝖤𝖥𝖦𝖧𝖨𝖩𝖪𝖫𝖬𝖭𝖮𝖯𝖰𝖱𝖲𝖳𝖴𝖵𝖶𝖷𝖸𝖹𝖺𝖻𝖼𝖽𝖾𝖿𝗀𝗁𝗂𝗃𝗄𝗅𝗆𝗇𝗈𝗉𝗊𝗋𝗌𝗍𝗎𝗏𝗐𝗑𝗒𝗓")))
    ).await?;

    Ok(())
}

#[poise::command(slash_command, install_context = "User", interaction_context = "Guild|BotDm|PrivateChannel", check="is_owner")]
async fn monospace(ctx: Context<'_>, content: String) -> Result<(), Error> {
    ctx.send(CreateReply::default()
        .ephemeral(true)
        .content(format!("```\n{}\n```", convert_to(&content, "𝙰𝙱𝙲𝙳𝙴𝙵𝙶𝙷𝙸𝙹𝙺𝙻𝙼𝙽𝙾𝙿𝚀𝚁𝚂𝚃𝚄𝚅𝚆𝚇𝚈𝚉𝚊𝚋𝚌𝚍𝚎𝚏𝚐𝚑𝚒𝚓𝚔𝚕𝚖𝚗𝚘𝚙𝚚𝚛𝚜𝚝𝚞𝚟𝚠𝚡𝚢𝚣")))
    ).await?;

    Ok(())
}

pub fn register() -> Vec<Command<Data, Error>> {
    vec![
        text()
    ]
}
