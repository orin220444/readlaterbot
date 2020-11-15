extern crate dotenv;
use dotenv::dotenv;
use teloxide::prelude::*;
mod link_finder;
mod post;
use post::post::Post;
#[tokio::main]
async fn main() {
    dotenv().ok();
    run_bot().await;
}

async fn run_bot() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        println!("{:#?}", &message.update.kind);
        let urls = link_finder::link_finder::link_finder(&message);
        match urls {
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
            None => println!("No urls!"),


        }

        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    })
    .await;
}
