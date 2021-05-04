use crate::keyboards::unarchive_keyboard;
use crate::Post;
use anyhow::Result;
use teloxide::prelude::*;

pub async fn archive(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    Post::archive_post(data).await?;
    let message_id = cx.update.message.clone().unwrap().id;
    let chat_id = cx.update.message.clone().unwrap().chat_id();
    cx.requester
        .answer_callback_query(cx.update.id)
        .text("Archived!")
        .await?;
    cx.requester
        .edit_message_text(
            chat_id,
            message_id,
            format!("{} [Archived]", cx.update.message.unwrap().text().unwrap()),
        )
        .await?;
    cx.requester
        .edit_message_reply_markup(chat_id, message_id)
        .reply_markup(unarchive_keyboard(data.parse::<i64>().unwrap()))
        .await?;
    Ok(())
}
