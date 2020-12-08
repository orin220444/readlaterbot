use crate::Post;
use rand::seq::SliceRandom;
use teloxide::prelude::*;
use crate::keyboards;
pub async fn random(cx: UpdateWithCx<Message>) {
    match Post::get_unarchived_posts().await {
        Err(e) => {
            println!("{:#?}", &e)
        }
        Ok(posts) => {
            let random_post_opt = posts.choose(&mut rand::thread_rng());
            println!("{:#?}", &random_post_opt);
            match random_post_opt {
                None => {
                    println!("error while getting random post");
                }
                Some(random_post) => {
                    cx.answer(/*format!("{:#?}", */ &random_post.original_url)
                        .reply_markup(keyboards::standart_keyboard())
                        /*)*/
                        .send()
                        .await;
                }
            }
        }
    }
}
