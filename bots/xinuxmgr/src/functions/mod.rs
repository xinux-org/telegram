pub mod about;
pub mod check;
pub mod help;
pub mod inline;
pub mod joined;
pub mod rules;
pub mod start;
pub mod warn;

pub use inline::inline;

use crate::utils::topics::Topics;
use crate::Command;
use std::error::Error;
use teloxide::{prelude::*, types::*};

pub async fn commands(
    bot: Bot,
    me: Me,
    msg: Message,
    cmd: Command,
    topics: Topics,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let _ = match cmd {
        Command::Start => crate::functions::start::command(&bot, &msg).await,
        Command::Help => crate::functions::help::command(&bot, &msg, &cmd).await,
        Command::Rules => crate::functions::rules::command(&bot, &msg).await,
        Command::About => crate::functions::about::command(&bot, &msg).await,
        Command::Warn => crate::functions::warn::command(&bot, &msg, &me, &topics).await,
        Command::Check => crate::functions::check::command(&bot, &msg).await,
    };

    Ok(())
}

pub async fn callback(
    bot: Bot,
    q: CallbackQuery,
    topics: Topics,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut args: Vec<&str> = Vec::new();

    if let Some(data) = q.data.clone() {
        if data.contains('_') {
            args = data.split('_').collect();
        } else {
            args.push(&data);
        }

        let _ = match args.remove(0) {
            "warn" => warn::callback(&bot, &q, &args, &topics).await,
            _ => Ok(()),
        };
    }

    Ok(())
}

pub async fn triggers(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(thread) = msg.thread_id {
        if msg.chat.id.0 == -1001174263940 && thread.0.0 == 178654 {
            // Delete anything except image
            if msg.photo().is_some() || msg.document().is_some() {
                return Ok(());
            }

            // Yup, ditch it
            return match bot.delete_message(msg.chat.id, msg.id).await {
                Ok(_) => Ok(()),
                Err(_) => Ok(()),
            };
        }
    }

    if let Some(ref user) = msg.from {
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
