use crate::keyboards::unarchive_keyboard;

use anyhow::Result;
use teloxide::prelude::*;

pub async fn archive(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    archive_post(data).await?;
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
        .reply_markup(unarchive_keyboard(data))
        .await?;
    Ok(())
}
use crate::db::connect_to_db;
use mongodb::bson::{Document, doc};
async fn archive_post(id: &str)-> Result<()> {
    let db = connect_to_db().await?;
    let coll = db.collection::<Document>("posts");
    let query = doc!{
        "_id": id.clone()
    };
    let update = doc! {
        "$set": {
            "read": true,
        }
    };
    coll.update_one(query, update, None).await?;
    Ok(())
}
