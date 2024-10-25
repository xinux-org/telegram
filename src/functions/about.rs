use crate::private;
use orzklv::telegram::{keyboard::Keyboard, topic::Topics};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardMarkup, ParseMode},
};

static TEXT: &str = r#"
<b>Hurmatli foydalanuvchi!</b>

Bizning botimiz aktiv tarzda shakllantirib boriladi. Buning ustida esa bir necha avtor va dasturchilar turadi, ushbu havolalar orqali bizning sinovchilarimizdan biriga aylaning va biz bilan botimiz, hamda guruhimiz ishlatish qulayligini oshiring.
"#;

pub async fn command(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    private!(bot, msg);

    bot.send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}

pub fn keyboard() -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    keyboard
        .url("Ochiq Havolalar", "https://github.com/xinux-org/telegram")
        .unwrap()
}
