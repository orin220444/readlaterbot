use crate::Post;
use anyhow::Result;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};
use teloxide::{
    prelude::{Request, UpdateWithCx},
    types::{CallbackQuery, ChatId, ChatOrInlineMessage, MediaKind, MessageKind},
};
pub async fn unarchive(cx: UpdateWithCx<CallbackQuery>, data: &str) -> Result<()> {
    let archive_keyboard = InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::new(
            "Delete",
            InlineKeyboardButtonKind::CallbackData("del".into()),
        ),
        InlineKeyboardButton::new(
            "Archive",
            InlineKeyboardButtonKind::CallbackData("archive".into()),
        ),
    ]]);
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
    cx.bot
        .answer_callback_query(cx.update.id)
        .text("Unarchived!")
        .send()
        .await?;
    cx.bot
        .edit_message_text(
            ChatOrInlineMessage::Chat {
                chat_id: ChatId::Id(chat_id),
                message_id: message_id,
            },
            text,
        )
        .send()
        .await?;
    cx.bot
        .edit_message_reply_markup(ChatOrInlineMessage::Chat {
            chat_id: ChatId::Id(chat_id),
            message_id: message_id,
        })
        .reply_markup(archive_keyboard)
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
