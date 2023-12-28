pub mod about;
pub mod check;
pub mod feedback;
pub mod help;
pub mod joined;
pub mod rules;
pub mod start;
pub mod warn;

use crate::Command;
use std::error::Error;
use teloxide::{prelude::*, types::*};

pub async fn commands(
    bot: Bot,
    me: Me,
    msg: Message,
    cmd: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let _ = match cmd {
        Command::Start => crate::functions::start::command(&bot, &msg).await,
        Command::Help => crate::functions::help::command(&bot, &msg, &cmd).await,
        Command::Rules => crate::functions::rules::command(&bot, &msg).await,
        Command::About => crate::functions::about::command(&bot, &msg).await,
        Command::Warn => crate::functions::warn::command(&bot, &msg, &me).await,
        Command::Check => crate::functions::check::command(&bot, &msg).await,
        Command::Feedback => crate::functions::feedback::command(&bot, &msg).await,
    };

    Ok(())
}

pub async fn callback(_bot: Bot, q: CallbackQuery) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut _args: Vec<&str> = Vec::new();

    if let Some(data) = q.data.clone() {
        if data.contains('_') {
            _args = data.split('_').collect();
        } else {
            _args.push(&data);
        }

        // let _ = match args.remove(0) {
        //     "group" => crate::functions::groups::callback_list(&bot, &q, &args, &groups).await,
        //     _ => Ok(()),
        // };
    }

    Ok(())
}

pub async fn triggers(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(thread) = msg.thread_id {
        if msg.chat.id.0 == -1001174263940 && thread == 178654 {
            match bot.delete_message(msg.chat.id, msg.id).await {
                Ok(_) => {}
                Err(_) => {}
            };
        }
    }

    if let Some(user) = msg.from() {
        if let Some(username) = user.username.clone() {
            if username == "Channel_Bot" {
                // Try to delete message and ignore error
                match bot.delete_message(msg.chat.id, msg.id).await {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }

    if let Some(new_chat_members) = msg.new_chat_members() {
        let bot_id = bot.get_me().send().await?.id;

        if !new_chat_members.iter().any(|user| user.id == bot_id)
            && (msg.chat.is_supergroup() || msg.chat.is_group())
        {
            crate::functions::joined::trigger(&bot, &msg).await?;
        }
    }

    Ok(())
}
