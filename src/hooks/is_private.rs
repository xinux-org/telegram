use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

use orzklv::telegram::{keyboard::Keyboard, timer::Timer, topic::Topics};

static TEXT: &str = "⚠️ <b>Bu komanda faqat shaxsiy chat uchun!</b>";

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard: Keyboard = Keyboard::new();
    keyboard
        .url("Shaxsiy Chat", "https://t.me/xinuxmgrbot")
        .unwrap()
}

pub async fn is_private(bot: &Bot, msg: &Message) -> ResponseResult<bool> {
    if msg.chat.is_private() {
        return Ok(true);
    }

    match bot.delete_message(msg.chat.id, msg.id).await {
        Ok(_) => {}
        Err(_) => {}
    };

    let message = bot
        .send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    bot.delete_timer(message.chat.id, message.id, 10)
        .await
        .await?;

    Ok(false)
}

#[macro_export]
macro_rules! private {
    ($bot:expr, $msg:expr) => {
        use $crate::hooks::is_private;

        if !is_private($bot, $msg).await.unwrap() {
            return Ok(());
        }
    };
}

pub use private;
