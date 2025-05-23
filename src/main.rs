use bot::bot::dispatch;
use bot::config::{Config, Field};
use bot::utils::{clog, resources::Resources, topics::Topics};
use bot::{Cli, Commands};
use clap::Parser;
use libxinux::pkgs::any::Any as Pkgs;
use std::error::Error;
use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Starter packs
    pretty_env_logger::init();
    log::info!("Starting Bot: {}", "xinuxmgr");

    // Global instances
    let topics = Topics::new();
    let pkgs = Pkgs::new().unwrap();
    let mut config = Config::default();
    let resources = Resources::new();

    // Dependencies
    let deps = dptree::deps![topics, pkgs, resources];

    // Args
    let args = Cli::parse();

    match args.command {
        Commands::Polling { token } => {
            match config.read(token, Field::Token) {
                Ok(_) => clog("Config", "Successfully read the token variable"),
                Err(e) => panic!("{}", e),
            };

            let bot = Bot::new(config.token);
            let mut dispatcher = dispatch(&bot, deps);

            clog("Mode", "starting polling on localhost");
            dispatcher.dispatch().await;

            Ok(())
        }
        Commands::Webhook {
            token,
            domain,
            port,
        } => {
            match config.read(token, Field::Token) {
                Ok(_) => clog("Config", "Successfully read the token variable"),
                Err(e) => panic!("{}", e),
            };

            match config.set(format!("https://{}", domain), Field::Domain) {
                Ok(_) => clog("Config", "Successfully set the domain variable"),
                Err(e) => panic!("{}", e),
            }

            let bot = Bot::new(config.token);
            let mut dispatcher = dispatch(&bot, deps);

            let addr = ([127, 0, 0, 1], port.unwrap_or(8445)).into(); // port 8445
            let listener = webhooks::axum(
                bot,
                webhooks::Options::new(addr, config.domain.parse().unwrap()),
            )
            .await
            .expect("Couldn't setup webhook");

            dispatcher
                .dispatch_with_listener(
                    listener,
                    LoggingErrorHandler::with_custom_text(
                        "An error has occurred in the dispatcher",
                    ),
                )
                .await;

            Ok(())
        }
        Commands::Env => {
            let bot = Bot::from_env();
            let mut dispatcher = dispatch(&bot, deps);

            match std::env::var("WEBHOOK_URL") {
                Ok(v) => {
                    clog("Mode", &format!("starting webhook on {}", v));

                    let port: u16 = std::env::var("PORT")
                        .unwrap_or("8445".to_string())
                        .parse()
                        .unwrap_or(8445);

                    let addr = ([0, 0, 0, 0], port).into();

                    let listener =
                        webhooks::axum(bot, webhooks::Options::new(addr, v.parse().unwrap()))
                            .await
                            .expect("Couldn't setup webhook");

                    dispatcher
                        .dispatch_with_listener(
                            listener,
                            LoggingErrorHandler::with_custom_text(
                                "An error has occurred in the dispatcher",
                            ),
                        )
                        .await;
                }
                Err(_) => {
                    clog("Mode", "starting polling on localhost");
                    dispatcher.dispatch().await;
                }
            }

            Ok(())
        }
    }
}
