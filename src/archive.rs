use teloxide::{
    types::{
        CallbackQuery, MediaKind, MessageKind
    },
    prelude::UpdateWithCx
    };
use crate::Post;
pub async fn archive(cx: UpdateWithCx<CallbackQuery>){
    let url:Option<&String> = {
        match &cx.update.message {
            None => {None},
            Some(message) => {
                match &message.kind {
            MessageKind::Common(message_common) => {
           match &message_common.media_kind {
               MediaKind::Text(message_text) => {Some(&message_text.text)}
               _ => {None}
           }
            },
            _ => {None},
        }

            }
        }
    };
    match  url {
    None => {},
    Some(url) => {

    Post::archive_post(url).await;
    }
    }
}
