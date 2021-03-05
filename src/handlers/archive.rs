use crate::Post;
use anyhow::Result;
use teloxide::{
    prelude::{Request, UpdateWithCx},
    types::{CallbackQuery, ChatId, ChatOrInlineMessage, MediaKind, MessageKind},
};
pub async fn archive(cx: UpdateWithCx<CallbackQuery>) -> Result<()> {
    let url: Option<&String> = {
        match &cx.update.message {
            None => None,
            Some(message) => match &message.kind {
                MessageKind::Common(message_common) => match &message_common.media_kind {
                    MediaKind::Text(message_text) => Some(&message_text.text),
                    _ => None,
                },
                _ => None,
            },
        }
    };
    match url {
        None => Ok(()),
        Some(url) => {
            Post::archive_post(url).await?;
            let message_id = cx.update.message.clone().unwrap().id;
            let chat_id = cx.update.message.clone().unwrap().chat_id();
            cx.bot
                .answer_callback_query(cx.update.id)
                .text("Archived!")
                .send()
                .await?;
            cx.bot
                .edit_message_text(
                    ChatOrInlineMessage::Chat {
                        chat_id: ChatId::Id(chat_id),
                        message_id: message_id,
                    },
                    format!("{} [Archived]", cx.update.message.unwrap().text().unwrap()),
                )
                .send()
                .await?;
            Ok(())
        }
    }
}
