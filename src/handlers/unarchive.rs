use crate::keyboards::standart_keyboard;
use crate::Post;
use anyhow::Result;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};
use teloxide::{
    prelude::*,
    types::{CallbackQuery, ChatId, MediaKind, MessageKind},
};
pub async fn unarchive(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    Post::unarchive_post(data).await?;
    let inline_message_id = cx.update.inline_message_id.clone().unwrap();
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
        .send()
        .await?;
    cx.requester
        .edit_message_text_inline(
inline_message_id.clone(),
            text,
        )
        .send()
        .await?;
    cx.requester
        .edit_message_reply_markup_inline(inline_message_id.clone())
        .reply_markup(standart_keyboard(data))
        .send()
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
