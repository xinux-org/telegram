use crate::{functions::start::keyboard, Command};
use orzklv::telegram::topic::Topics;
use teloxide::{payloads::SendMessageSetters, prelude::*, types::ParseMode};

static TEXT: &[(&str, &str)] = &[
    ("help", "ushbu xabarni qayta ko'rsatish"),
    ("rules", "qoidalarni aks ettirish"),
    ("check", "chaqirilgan joydan parametrlarni ko'rish"),
    ("about", "ushbu botimizning rivojlantirish qismi"),
    ("feedback", "ushbu bot haqida fikr va taklif qoldirish"),
    (
        "warn",
        "reply qilingan odamga offtop borligi haqida eslatish",
    ),
];

pub async fn command(bot: &Bot, msg: &Message, _cmd: &Command) -> ResponseResult<()> {
    let mut text = String::new();

    text.push_str("<b>Mavjud komandalar ro'yxati:</b>\n\n");

    for cmd in TEXT {
        text.push('/');
        text.push_str(cmd.0);
        text.push_str(" - ");
        text.push_str(format!("<code>{text}</code>", text = cmd.1).as_str());
        text.push('\n');
    }

    bot.send_message_tf(msg.chat.id, text, msg)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard())
        .await?;

    Ok(())
}
