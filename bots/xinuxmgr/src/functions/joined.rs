use teloxide::{prelude::*, types::*};

use crate::utils::message::{delete_timer, Assistant};

static TEXT: &str = "<b>Salom Administrator!</b>\n\n\
Sizni ko'rib turganimdan bag'oyatda xursandman.
Men Xinux Jamiyati tomonidan yaratilgan bot hisoblanib,
Xinux Jamiyati foydalanuvchilari uchun foydali resurslarni yetkazish, saqlash va
ularni saralash uchun xizmat qilaman.
";

pub async fn trigger(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let message = bot
        .send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .await?;

    delete_timer(bot, &message, 60 * 5).await?;

    Ok(())
}
