use crate::Post;
use anyhow::Result;
use teloxide::prelude::*;
pub async fn delete(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    Post::delete_post(data).await?;
    cx.requester
        .answer_callback_query(cx.update.id)
        .text("Deleted!")
        .await?;
    if let Some(message) = cx.update.message {
        println!("test");
        cx.requester
            .delete_message(message.chat.id, message.id)
            .await?;
    }
    Ok(())
}
