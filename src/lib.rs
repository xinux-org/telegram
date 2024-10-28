#![allow(clippy::single_match)]

pub mod bot;
pub mod config;
pub mod functions;
pub mod hooks;
pub mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Telegram bot manager for Xinux community
#[derive(Debug, Parser)]
#[command(name = "bot")]
#[command(about = "Telegram bot manager for Xinux community", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Start bot in Polling mode with token
    #[command(arg_required_else_help = true)]
    Polling {
        /// Telegram bot token
        #[arg(required = true)]
        token: PathBuf,
    },
    /// Start bot in Webhook mode with given variables
    #[command(arg_required_else_help = true)]
    Webhook {
        /// Telegram bot token
        #[arg(required = true)]
        token: PathBuf,

        /// Domain url to set webhook address
        #[arg(required = true)]
        domain: String,
    },
    /// Start bot by getting necessary configurations from environmental variables
    Env,
}
