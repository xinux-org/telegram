use crate::utils::{keyboard::Keyboard, message::Assistant};
use teloxide::{prelude::*, types::*};
use teloxide::payloads::AnswerCallbackQuery;
use crate::utils::topics::Topics;

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

    bot.send_message_tf(msg.chat.id, "Qaysi mavzu taraflama yozgan odam chetlashdi?", msg) // view_detail(msg.reply_to_message().unwrap())
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard(topics.list(), msg.from().unwrap().id))
        .await?;

    Ok(())
}

pub async fn callback(
    bot: &Bot,
    q: &CallbackQuery,
    args: &Vec<&str>,
    topics: &Topics,
) -> ResponseResult<()> {
    println!("{:?}", args);

    if q.from.id != UserId(args[0].parse::<u64>().unwrap()) {
        bot.answer_callback_query(q.id.clone())
            .text("You're not the one to answer this!")
            .show_alert(true)
            .send()
            .await?;

        println!("Sent");

        return Ok(());
    }

    let title = args[1];
    let code = topics.get(title.clone());
    let message = q.message.clone().unwrap();

    match code {
        None => {

            bot.delete_message(message.chat.id, message.id).await?;
            bot.send_message_tf(
                message.chat.id,
                "Unaqa topic borga o'xshamaydi do'stlar...",
                &message
            ).await?;

            return Ok(())
        }
        Some(c) => {
            bot.delete_message(message.chat.id, message.id).await?;

            bot.send_message_tf(
                message.chat.id,
                view_detail(&message),
                &message
            )
                .reply_markup()
                .parse_mode(ParseMode::Html)
                .await?;

            return Ok(())
        }
    }

    Ok(())
}

pub fn view_detail(msg: &Message) -> String {
    format!(
        "<b>Hurmatli <a href=\"tg://user?id={}\">{}</a>,</b>\
        \n\n\
        Tushunishim bo'yicha siz mavzudan chetlayashayabsiz. Iltimos, \
        quyidagi tugmachani bosish orqali bizning offtop guruhga o'tib oling! \
        Offtopic guruhimizda istalgan mavzuda suhbatlashish ruxsat etiladi. Boshqalarga halaqit qilmayliga 😉\
        \n\n\
        <b>Hurmat ila, Xinux Menejer</b>",
        msg.from().unwrap().id,
        msg.from().unwrap().first_name
    )
}

pub fn keyboard(list: Vec<String>, owner: UserId) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();

    for (index, topic) in list.iter().enumerate() {
        keyboard.text(&topic, &format!("warn_{}_{}", owner.0, topic));

        if index % 2 == 1 {
            keyboard.row();
        }
    }

    keyboard.get()
}

pub fn callback_keyboard(title: String) -> InlineKeyboardMarkup {
    let mut keyboard = Keyboard::new();
    let capitalized
    keyboard.url(format!("{} Chat"));

    keyboard.get()
}
