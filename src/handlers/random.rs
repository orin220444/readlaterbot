use crate::keyboards;
use crate::Post;
use anyhow::Result;
use rand::seq::SliceRandom;
use teloxide::prelude::*;
pub async fn random(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
    let posts = Post::get_unarchived_posts(Post::default()).await?;
    let random_post_opt = posts.choose(&mut rand::thread_rng());
    println!("{:#?}", &random_post_opt);
    match random_post_opt {
        None => {
            println!("error while getting random post");
            Ok(())
        }
        Some(random_post) => {
            cx.answer(&random_post.original_url)
                .reply_markup(keyboards::standart_keyboard(&format!(
                    "{}",
                    random_post.id()
                )))
                .send()
                .await?;
            Ok(())
        }
    }
}
