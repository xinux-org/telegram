use crate::utils::{keyboard::Keyboard, message::Assistant};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Assalomu alaykum, hurmatli xinux a'zosi!</b>

Sizni ko'rib turganimdan bag'oyatda xursandman. Men O'zbek Linux rivojlantirish hamjamiyatlardan birining asistenti bo'laman va ko'pincha holatlarda ushbu jamiyatlarni boshqarish uchun xizmat qilaman.
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard.url("Jamiyat", "https://t.me/xinuxuz");
    keyboard.url("Web Sahifa", "https://www.xinux.uz")
}
