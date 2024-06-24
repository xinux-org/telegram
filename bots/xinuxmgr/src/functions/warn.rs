use crate::utils::topics::Topics;
use orzklv::{
    string::Transform,
    telegram::{keyboard::Keyboard, topic::Topics as TopicsTrait},
};
use std::fmt::Display;
use teloxide::{prelude::*, types::*};

static TEXT_FAIL: &str = "Ha-ha... yaxshi urinish!";
static TEXT_NON_REPLY: &str = "↪ Reply bilan ko'rsatingchi habarni!";
static NON_XINUX: &str = "Ebe hay, biz Xinux guruhida emasga o'xshaymiz...";

pub async fn command(bot: &Bot, msg: &Message, me: &Me, topics: &Topics) -> ResponseResult<()> {
    if msg.chat.id != ChatId(-1001174263940) {
        return {
            bot.send_message_tf(msg.chat.id, NON_XINUX, msg).await?;
            Ok(())
        };
    }

    let attempt = bot.delete_message(msg.chat.id, msg.id).await;
    match attempt {
        Ok(_) => {}
        Err(_) => {
            bot.send_message_tf(
                msg.chat.id,
                "Ebe hay, men habarlar o'chirish uchun yetarlicha imtiyozim yo'q!",
                msg,
            )
            .await?;
            return Ok(());
        }
    }

    if msg.reply_to_message().is_none()
        || msg.reply_to_message().unwrap().id == MessageId(msg.thread_id.unwrap())
    {
        return {
            bot.send_message_tf(msg.chat.id, TEXT_NON_REPLY, msg)
                .await?;
            Ok(())
        };
    }

    // if replied person is bot itself, send a fail message
    if let Some(user) = msg.reply_to_message().as_ref().unwrap().from() {
        if user.username.is_some() && user.username.clone().unwrap() == me.username() {
            return {
                bot.send_message_tf(msg.chat.id, TEXT_FAIL, msg).await?;
                Ok(())
            };
        }
    }

    bot.delete_message(msg.chat.id, msg.reply_to_message().unwrap().id)
        .await?;

    let replied_person = match msg.reply_to_message().unwrap().from() {
        None => {
            bot.send_message_tf(
                msg.chat.id,
                "Hmmm, qiziq odam ekan reply qilingan odam...",
                msg,
            )
            .await?;

            return Ok(());
        }
        Some(p) => p,
    };

    bot.send_message_tf(
        msg.chat.id,
        format!(
            "<b>Xo'sh, <a href=\"tg://user?id={}\">{}</a>.</b> Qaysi mavzu taraflama yozgan odam chetlashdi?",
            msg.from().unwrap().id,
            msg.from().unwrap().first_name
        ),
        msg,
    ) // view_detail(msg.reply_to_message().unwrap())
    .parse_mode(ParseMode::Html)
    .reply_markup(keyboard(
        topics.list(),
        msg.from().unwrap().id,
        &replied_person.id,
        &replied_person.first_name,
    ))
    .await?;

    Ok(())
}

pub async fn callback(
    bot: &Bot,
    q: &CallbackQuery,
    args: &[&str],
    topics: &Topics,
) -> ResponseResult<()> {
    if q.from.id != UserId(args[0].parse::<u64>().unwrap()) {
        bot.answer_callback_query(q.id.clone())
            .text("Sen chaqirmadingku komandani! Nimaga o'z boshimchalik qilayabsan...")
            .show_alert(true)
            .send()
            .await?;

        return Ok(());
    }

    let title = args[1];
    let code = topics.get(title);
    let message = q.message.clone().unwrap();
    let sender = (args[2], args[3]);

    match code {
        None => {
            bot.delete_message(message.chat.id, message.id).await?;
            bot.send_message_tf(
                message.chat.id,
                "Unaqa topic borga o'xshamaydi do'stlar...",
                &message,
            )
            .await?;

            Ok(())
        }
        Some(c) => {
            bot.delete_message(message.chat.id, message.id).await?;

            bot.send_message_tf(
                message.chat.id,
                view_detail(sender, title.to_string()),
                &message,
            )
            .reply_markup(callback_keyboard(title, c))
            .parse_mode(ParseMode::Html)
            .await?;

            Ok(())
        }
    }
}

pub fn view_detail(from: (&str, &str), topic: String) -> String {
    format!(
        "<b>Hurmatli <a href=\"tg://user?id={}\">{}</a>,</b>\
        \n\n\
        Tushunishim bo'yicha siz mavzudan chetlayashayabsiz. Iltimos, \
        quyidagi tugmachani bosish orqali bizning {} guruhga o'tib oling! \
        {} guruhimizda ushbu mavzuga oid narsalar haqida suhbatlashish ruxsat etiladi. \
        Boshqalarga halaqit qilmayliga 😉\
        \n\n\
        <b>Hurmat ila, Xinux Menejer</b>",
        from.0,
        from.1,
        topic,
        topic.capitalize()
    )
}

pub fn keyboard<T>(
    list: Vec<String>,
    owner: UserId,
    replied: &UserId,
    name: T,
) -> InlineKeyboardMarkup
where
    T: AsRef<str> + Display,
{
    let mut keyboard = Keyboard::new();

    for (index, topic) in list.iter().enumerate() {
        keyboard.text(
            topic,
            &format!("warn_{}_{}_{}_{}", owner.0, topic, replied.0, name),
        );

        if index % 2 == 1 {
            keyboard.row();
        }
    }

    keyboard.get()
}

pub fn callback_keyboard<T>(title: T, topic: &u32) -> InlineKeyboardMarkup
where
    T: AsRef<str> + Display + ToString,
{
    let mut keyboard = Keyboard::new();

    let url: String = match topic {
        0 => "https://t.me/flossuzc/115778".to_string(),
        _ => format!("https://t.me/xinuxuz/{}", topic),
    };

    keyboard
        .url(
            &format!("{} Chat", title.to_string().capitalize()),
            &url.to_string(),
        )
        .unwrap()
}
