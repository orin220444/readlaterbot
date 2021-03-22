use crate::Post;
use anyhow::Result;
use teloxide::{
    prelude::*,
    types::{CallbackQuery, MediaKind, MessageKind},
};
pub async fn delete(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    Post::delete_post(data).await?;
    cx.requester
        .answer_callback_query(cx.update.id)
        .text("Deleted!")
        .send()
        .await?;
    if let Some(message) = cx.update.message {
        println!("test");
        cx.requester
            .delete_message(message.chat.id, message.id)
            .send()
            .await?;
    }
    Ok(())
}
