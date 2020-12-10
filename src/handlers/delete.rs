use crate::Post;
use teloxide::{
    prelude::{Request, UpdateWithCx},
    types::{CallbackQuery, MediaKind, MessageKind},
};
pub async fn delete(cx: UpdateWithCx<CallbackQuery>) {
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
        None => {}
        Some(url) => {
            Post::delete_post(url).await;
            cx.bot
                .answer_callback_query(cx.update.id)
                .text("Deleted!")
                .send()
                .await;
            if let Some(message) = cx.update.message {
                println!("test");
                cx.bot
                    .delete_message(message.chat.id, message.id)
                    .send()
                    .await;
            }
        }
    }
}
