use crate::keyboards;
use crate::Post;
use anyhow::Result;
use teloxide::prelude::*;

pub async fn random(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
    let doc_count = count_unread_posts().await?;
    if doc_count == 0 {
        all_posts_viewed(&cx).await?;
    } else {
        let random_id = gen_random_number(doc_count);
    let post = get_random_post(random_id).await?;
    cx.answer(&post.original_url)
        .reply_markup(keyboards::standart_keyboard(&post.id()))
        .await?;
    }
    Ok(())
}
use crate::db::connect_to_db;
use futures::stream::StreamExt;
use mongodb::bson::{doc, from_document, Document};
use rand::Rng;

pub async fn get_random_post(random_id: u64) -> Result<Post> {
    let db = connect_to_db().await?;
    let _coll = db.collection::<Document>("posts");
    let match_aggr = doc! {
        "$match": {
            "read": false
    }
    };
    let limit_aggr = doc! {
    "$limit" : 1
    };
    let skip_aggr = doc! {
        "$skip" : random_id
    };
    let pipeline = [match_aggr, skip_aggr, limit_aggr,];
    let mut cursor = _coll.aggregate(pipeline, None).await?;
    let opt_doc = cursor.next().await;
    // TODO: fix this shit
    let res_doc = opt_doc.unwrap_or_else(|| panic!("Can't get doc from cursor!"));
    let doc = res_doc?;
    let res = from_document::<Post>(doc)?;
    Ok(res)
}
fn gen_random_number(doc_count: u64) -> u64 {
    let mut rng = rand::thread_rng();
    
    rng.gen_range(0..doc_count)
}
async fn all_posts_viewed(cx: &UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
    cx
    .answer("All unread posts vieved!")
    .await?;
    Ok(())
}
async fn count_unread_posts() -> Result<u64> {
    let db = connect_to_db().await?;
    let _coll = db.collection::<Document>("posts");
    let filter = doc! {
        "read": false
    };
    let doc_count = _coll.count_documents(filter, None).await?;
    Ok(doc_count)
}
