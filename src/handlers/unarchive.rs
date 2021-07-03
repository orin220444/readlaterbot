use crate::keyboards::standart_keyboard;

use anyhow::Result;
use teloxide::prelude::*;

pub async fn unarchive(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    unarchive_post(data).await?;
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
use crate::db::connect_to_db;
//use crate::post::Post;
use mongodb::{bson::{doc, oid::ObjectId}};
use mongodb::bson::Document;
async fn unarchive_post(id: &str) -> Result<()> {
    let obj_id = ObjectId::parse_str(id)?;
    let db = connect_to_db().await?;
    let coll = db.collection::<Document>("posts");
    let query = doc!{
        "_id": obj_id,
    };
    let update = doc! {
        "$set": {
            "read": false
        }
    };
    coll.update_one(query, update, None).await?;
    Ok(())
}
