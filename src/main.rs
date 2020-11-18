extern crate dotenv;
use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommand};
mod link_finder;
mod post;
mod random;
use post::post::Post;
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
            cx.answer("test").send().await?;
            crate::random::random(cx).await;
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
                let urls = link_finder::link_finder::link_finder(&cx);
                match urls {
                    None => println!("No urls!"),
                    Some(urls) => {
                        println!("{:#?}", urls);
                        for url in urls {
                            let mut post = Post::new(url);
                            post.real_url().await;
                            match post.save_post().await {
                                Ok(_) => log::info!("Successful saved post"),
                                Err(e) => println!("{:#?}", e),
                            }
                        }
                    }
                }

                cx.answer_dice().send().await?;
                ResponseResult::<()>::Ok(())
            }
        }
    }
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
        .dispatch()
        .await;
}
