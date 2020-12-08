extern crate dotenv;
use dotenv::dotenv;
use teloxide::{
    prelude::ResponseResult,
    prelude::{Dispatcher, DispatcherHandlerRx, Request, StreamExt, UpdateWithCx},
    types::{CallbackQuery, Message},
    utils::command::BotCommand,
    Bot,
};
mod handlers;
mod link_finder;
mod post;
mod keyboards;
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
async fn command_answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Random => {
            crate::handlers::random::random(cx).await;
            ResponseResult::<()>::Ok(())
        }
    }
}
async fn handle_message(cx: UpdateWithCx<Message>) -> ResponseResult<()> {
    match cx.update.text() {
        None => ResponseResult::<()>::Ok(()),
        Some(text) => {
            if let Ok(command) = Command::parse(text, "test_name_bot") {
                command_answer(cx, command).await?;
                ResponseResult::<()>::Ok(())
            } else {
                println!("{:#?}", &cx.update.kind);
                let urls = link_finder::link_finder(&cx);
                match urls {
                    None => println!("No urls!"),
                    Some(urls) => {
                        println!("{:#?}", urls);
                        for url in urls {
                            let mut post = Post::new(&url);
                            post.real_url().await;
                            match post.save_post().await {
                                Ok(_) => {
                                    log::info!("Successful saved post");
                                    match cx.answer(url).reply_markup(keyboards::standart_keyboard())
                                        .send().await {
                                        Ok(_) => {}
                                        Err(e) => {
                                            println!("Error while sending linkfinder url! {:#?}", e)
                                        }
                                    }
                                }
                                Err(e) => println!("{:#?}", e),
                            }
                        }
                    }
                }

                ResponseResult::<()>::Ok(())
            }
        }
    }
}
async fn handle_callback_query(cx: UpdateWithCx<CallbackQuery>) -> ResponseResult<()> {
    let data = &cx.update.data;
    match data {
        None => {}
        Some(data) => {
            // TODO: ref using enums
            if data == "del" {
                crate::handlers::delete::delete(cx).await;
            } else if data == "archive" {
                crate::handlers::archive::archive(cx).await;
            }
        }
    }
    Ok(())
}
async fn run_bot() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();
    //.await;

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, |cx| async move {
                match handle_message(cx).await {
                    Ok(_) => {}
                    Err(e) => println!("Error while handling messages: {:#?}", e),
                };
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<CallbackQuery>| {
            rx.for_each_concurrent(None, |cx| async move {
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
