use crate::hooks;
use orzklv::telegram::keyboard::Keyboard;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static ROADMAP: &str = include_str!("../../data/roadmap.md");

static LINKS: &[(&str, &str)] = &[
    ("Xinux Rasmiy Qo'llanmalari", "https://xinux.uz/learn"),
    (
        "Nix da birinchi qadam",
        "https://nix.dev/tutorials/first-steps/",
    ),
    ("0 dan Nix gacha", "https://zero-to-nix.com"),
];

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    if !hooks::is_private(bot, msg).await.unwrap() {
        return Ok(());
    }

    bot.send_message(msg.chat.id, ROADMAP)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for link in LINKS {
        keyboard.url(link.0, link.1).unwrap();
        keyboard.row();
    }

    keyboard.get()
}
