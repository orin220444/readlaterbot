extern crate dotenv;
use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommand};
mod link_finder;
mod post;
use post::post::Post;
#[tokio::main]
async fn main() {
    dotenv().ok();
    run_bot().await;
}
#[derive(BotCommand)]
enum Command {
    Random,
}
async fn command_answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()>{
match command {
    Command::Random => { cx.answer("test").send().await;
Ok(())
    }
}
}
async fn run_bot() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();
/*let cloned_bot = bot.clone();
    teloxide::commands_repl(cloned_bot, "test_bot_name", command_answer).await;
    teloxide::repl(bot, |message| async move {
        println!("{:#?}", &message.update.kind);
        let urls = link_finder::link_finder::link_finder(&message);
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

        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    })
    .await;
  */
    Dispatcher::new(bot).messages_handler(|rx: DispatcherHandlerRx<Message>|{
    rx.for_each_concurrent(None, |message| async move {
        println!("{:#?}", message);
    })});
}
