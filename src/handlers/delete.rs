use crate::Post;
use anyhow::Result;
use teloxide::{
    prelude::{Request, UpdateWithCx},
    types::{CallbackQuery, MediaKind, MessageKind},
};
pub async fn delete(cx: UpdateWithCx<CallbackQuery>, data: &str) -> Result<()> {
    Post::delete_post(data).await?;
    cx.bot
        .answer_callback_query(cx.update.id)
        .text("Deleted!")
        .send()
        .await?;
    if let Some(message) = cx.update.message {
        println!("test");
        cx.bot
            .delete_message(message.chat.id, message.id)
            .send()
            .await?;
    }
    Ok(())
}
