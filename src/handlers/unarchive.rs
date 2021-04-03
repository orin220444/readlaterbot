use crate::keyboards::standart_keyboard;
use crate::Post;
use anyhow::Result;
use teloxide::prelude::*;

pub async fn unarchive(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    Post::unarchive_post(data).await?;
    let message_id = cx.update.message.clone().unwrap().id;
    let chat_id = cx.update.message.clone().unwrap().chat_id();
    let text = clear_label(
        cx.update
            .message
            .clone()
            .unwrap()
            .text()
            .unwrap()
            .to_string(),
    );
    cx.requester
        .answer_callback_query(cx.update.id)
        .text("Unarchived!")
        .await?;
    cx.requester
        .edit_message_text(chat_id, message_id, text)
        .await?;
    cx.requester
        .edit_message_reply_markup(chat_id, message_id)
        .reply_markup(standart_keyboard(data))
        .await?;
    Ok(())
}

fn clear_label(text: String) -> String {
    if text.contains("[Archived]") {
        text.strip_suffix("[Archived]").unwrap().to_string()
    } else {
        text
    }
}
