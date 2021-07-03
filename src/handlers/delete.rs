use crate::{db::connect_to_db};
use anyhow::Result;
use teloxide::prelude::*;
pub async fn delete(cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>, data: &str) -> Result<()> {
    delete_post(data).await?;
    cx.requester
        .answer_callback_query(cx.update.id)
        .text("Deleted!")
        .await?;
    if let Some(message) = cx.update.message {
        println!("test");
        cx.requester
            .delete_message(message.chat.id, message.id)
            .await?;
    }
    Ok(())
}
use mongodb::bson::{Document, doc, oid::ObjectId};
async fn delete_post(id: &str) -> Result<()> {
    let obj_id = ObjectId::parse_str(id)?;
    let db = connect_to_db().await?;
    let coll = db.collection::<Document>("posts");
    let query = doc!{
        "_id": obj_id
    };
    coll.delete_one(query, None).await?;
    Ok(())
}
