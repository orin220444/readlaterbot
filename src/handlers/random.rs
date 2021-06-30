use crate::keyboards;
use crate::Post;
use anyhow::Result;

use teloxide::prelude::*;
pub async fn random(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
    let post = get_random_post().await?;
            cx.answer(&post.original_url)
                .reply_markup(keyboards::standart_keyboard(&post.id()))
                .await?;
            Ok(())
        }
use mongodb::{bson::{Document, doc, from_document}};
use crate::db::connect_to_db;
use futures::stream::{StreamExt};
pub async fn get_random_post() -> Result<Post> {
let db = connect_to_db().await?;
let _coll = db.collection::<Document>("posts");
let match_aggr = doc!{

    "$match": {
    "read": false
}
};
let sample_aggr = doc!{

        "$sample": {
            "size": 1
}

};
let pipeline = [ match_aggr, sample_aggr,];
let mut cursor = db.aggregate(pipeline, None).await?;
//let res = cursor.collect::<Vec<Document>>().await;
while let Some(doc) = cursor.next().await {
    if let Ok(doc) = doc {
    println!("{:#?}", doc)
    }
}
let opt_doc = cursor.next().await;
// TODO: fix this shit
let res_doc = opt_doc.unwrap_or_else(|| panic!("Can't get doc from cursor!"));
let doc = res_doc?;
let res = from_document::<Post>(doc)?;
Ok(res)
}
