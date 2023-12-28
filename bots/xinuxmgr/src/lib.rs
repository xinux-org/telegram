#![allow(clippy::single_match)]

pub mod functions;
pub mod hooks;
pub mod utils;

use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", parse_with = "split")]
#[command(description = "These are the commands that I can understand:")]
pub enum Command {
    /// List existing commands
    Help,

    /// Starting point of the bot
    Start,

    /// Rules of our chat
    Rules,

    /// About the bot
    About,

    /// Report offtopic
    Warn,

    /// Check for chatid
    Check,

    /// Leave a feedback for the bot
    Feedback,
}

pub fn handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dptree::entry()
        // Callbacks
        .branch(Update::filter_callback_query().endpoint(functions::callback))
        // Commands
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(functions::commands),
        )
        .branch(Update::filter_message().endpoint(functions::triggers))
}
