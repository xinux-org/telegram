use teloxide::{prelude::*, types::*};

use crate::utils::message::{delete_timer, Assistant};

static TEXT: &str = "<b>Salom Administrator!</b>

Sizni ko'rib turganimdan bag'oyatda xursandman. Men \
Xinux Jamiyati tomonidan yaratilgan bot hisoblanib, \
Xinux Jamiyati foydalanuvchilari uchun foydali resurslarni \
yetkazish, saqlash va ularni saralash uchun xizmat qilaman. \
Ushbu guruhda esa Linux ga oid istagan mavzuda gaplashish, \
ma'lumot yoki tajriba ulashish mumkin.

Agar siz bu yerlarga yordam axtarib kelgan bo'lsangiz, \
<a href=\"https://t.me/xinuxuz/178646\">yordam kanali</a> \
ga muroojat qilishingizni so'rab qolamiz. Hamda, qoidalarni \
/rules buyrug'i orqali o'qib olish esingizdan chiqmasin. \
Keyin yana adminlar jazolashsa hayron bo'lib yurmang!
";

pub async fn trigger(bot: &Bot, msg: &Message) -> ResponseResult<()> {
    let message = bot
        .send_message_tf(msg.chat.id, TEXT, msg)
        .parse_mode(ParseMode::Html)
        .await?;

    delete_timer(bot, &message, 60 * 5).await?;

    Ok(())
}
