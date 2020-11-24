use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup},
};
pub async fn get_all_posts(cx: UpdateWithCx<Message>) {
    match crate::post::Post::get_all_posts().await {
        Ok(posts) => {
            let stop:usize = {
                if posts.len() >= 50 {
                    49
                }
                else {
                    posts.len()
                }
            };
            let mut button_posts = Vec::with_capacity(stop + 1);
            for post in &posts[0..stop] {
                button_posts.push(vec![InlineKeyboardButton::new(
                    &post.original_url,
                    InlineKeyboardButtonKind::CallbackData(String::from(&post.original_url)),
                )]);
            }
            button_posts.push(vec![
                InlineKeyboardButton::new(
                    "<-",
                    InlineKeyboardButtonKind::CallbackData("prev page".to_string()),
                ),
                InlineKeyboardButton::new(
                    "->",
                    InlineKeyboardButtonKind::CallbackData("next page".to_string()),
                ),
            ]);
            println!("{:#?}", button_posts);
            match cx
                .answer("All Posts".to_string())
                .reply_markup(ReplyMarkup::InlineKeyboardMarkup(
                    InlineKeyboardMarkup::new(button_posts),
                ))
                .send()
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    println!("{:#?}", e)
                }
            }
        }
        Err(e) => {
            println!("{:#?}", e)
        }
    }
}
