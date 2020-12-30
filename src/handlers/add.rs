use crate::keyboards;
use crate::link_finder;
use crate::post::Post;
use anyhow::Result;
use teloxide::{
    prelude::{Request, UpdateWithCx},
    types::Message,
};
pub async fn add(cx: UpdateWithCx<Message>) -> Result<()> {
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
                        match cx
                            .answer(url)
                            .reply_markup(keyboards::standart_keyboard())
                            .send()
                            .await
                        {
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

    Ok(())
}
