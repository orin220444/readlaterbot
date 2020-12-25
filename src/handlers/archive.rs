use crate::Post;
use anyhow::Result;
use teloxide::{
    prelude::{Request, UpdateWithCx},
    types::{CallbackQuery, MediaKind, MessageKind},
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
            cx.bot
                .answer_callback_query(cx.update.id)
                .text("Archived!")
                .send()
                .await?;
            Ok(())
        }
    }
}
