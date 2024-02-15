use std::error::Error;
use teloxide::{prelude::*, update_listeners::webhooks};
use xinuxmgr::{handler, utils::cargo_like_log};
use xinuxmgr::utils::topics::Topics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    log::info!("Starting Bot: {}", "xinuxmgr");

    let bot = Bot::from_env();
    let topics = Topics::new();

    // Dispatcher flow control
    let mut dispatcher = Dispatcher::builder(bot.clone(), handler())
        .dependencies(dptree::deps![topics])
        // If no handler succeeded to handle an update, this closure will be called
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        // If the dispatcher fails for some reason, execute this handler
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build();

    match std::env::var("WEBHOOK_URL") {
        Ok(v) => {
            cargo_like_log("Mode", &format!("starting webhook on {}", v));
            let addr = ([0, 0, 0, 0], 8443).into(); // port 8443
            let listener = webhooks::axum(bot, webhooks::Options::new(addr, v.parse().unwrap()))
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
            cargo_like_log("Mode", "starting polling on localhost");
            dispatcher.dispatch().await;
        }
    }

    Ok(())
}
