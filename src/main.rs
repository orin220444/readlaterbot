use anyhow::Result;
use dotenv::dotenv;
use teloxide::{
    prelude::*,
    utils::command::BotCommand,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
mod db;
mod handlers;
mod keyboards;
mod link_finder;
mod post;
use post::Post;
#[tokio::main]
async fn main() {
    dotenv().ok();
    run_bot().await;
}
#[derive(BotCommand, Debug)]
#[command(rename = "lowercase")]
enum Command {
    Random,
}
async fn command_answer(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> Result<()> {
    match command {
        Command::Random => {
            crate::handlers::random(cx).await?;
            Ok(())
        }
    }
}
async fn handle_message(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
    match cx.update.text() {
        None => Ok(()),
        Some(text) => {
            if let Ok(command) = Command::parse(text, "test_name_bot") {
                command_answer(cx, command).await?;
                Ok(())
            } else {
                handlers::add(cx).await?;
                Ok(())
            }
        }
    }
}
async fn handle_callback_query(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>) -> Result<()> {
    let data = cx.update.data.clone();
    match data {
        None => {}
        Some(data) => {
            // TODO: ref using enums
            if data.starts_with("del") {
                crate::handlers::delete(cx, &data.strip_prefix("del").unwrap()).await?;
            } else if data.starts_with("archive") {
                crate::handlers::archive(cx, &data.strip_prefix("archive").unwrap()).await?;
            } else if data.starts_with("unarchive") {
                crate::handlers::unarchive(cx, &data.strip_prefix("unarchive").unwrap()).await?;
            }
        }
    }
    Ok(())
}
async fn run_bot() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = teloxide::Bot::new(std::env::var("TELEGRAM_BOT_TOKEN").unwrap()).auto_send();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx)
            .for_each_concurrent(None, |cx| async move {
                match handle_message(cx).await {
                    Ok(_) => {}
                    Err(e) => println!("Error while handling messages: {:#?}", e),
                };
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, CallbackQuery>| {
            UnboundedReceiverStream::new(rx)
            .for_each_concurrent(None, |cx| async move {
                println!("New Callback query: {:#?}", &cx.update);
                match handle_callback_query(cx).await {
                    Ok(_) => {}
                    Err(e) => println!("Error while handling Callback queries: {:#?}", e),
                };
            })
        })
        .dispatch()
        .await;
}
