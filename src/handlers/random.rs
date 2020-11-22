use crate::Post;
use rand::seq::SliceRandom;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup},
};
pub async fn random(cx: UpdateWithCx<Message>) {
    match Post::get_all_posts().await {
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
                        .reply_markup(ReplyMarkup::InlineKeyboardMarkup(
                            InlineKeyboardMarkup::new(vec![vec![
                                InlineKeyboardButton::new(
                                    "Delete",
                                    InlineKeyboardButtonKind::CallbackData("del".into()),
                                ),
                                InlineKeyboardButton::new(
                                    "Archive",
                                    InlineKeyboardButtonKind::CallbackData("archive".into()),
                                ),
                            ]]),
                        ))
                        /*)*/
                        .send()
                        .await;
                }
            }
        }
    }
}
