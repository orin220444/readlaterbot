use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup},
};
pub async fn get_all_posts(cx: UpdateWithCx<Message>) {
    match crate::post::Post::get_all_posts().await {
        Ok(posts) => {
            let mut button_posts = Vec::with_capacity(posts.len());
            for post in posts {
                button_posts.push(vec![InlineKeyboardButton::new(
                    &post.original_url,
                    InlineKeyboardButtonKind::CallbackData(String::from(&post.original_url)),
                )]);
            }
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
