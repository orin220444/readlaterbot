use crate::db::Db;
use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use futures::stream::StreamExt;
use mongodb::bson::Document;
use mongodb::bson::{de::from_document, doc, ser::to_document};
use serde_derive::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Post {
    id: i64,
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}

impl Post {
    async fn save_to_db(&self) -> Result<()> {
        let bson_post = to_document(&self)?;
        Ok(Db::insert_one("posts".to_string(), bson_post, None).await?)
    }
    pub async fn get_all_posts(self) -> Result<Vec<Post>> {
        let filter = doc! {};
        let mut bson_data = Db::find("posts".to_string(), filter, None).await?;
        let mut posts = Vec::new();
        for doc in bson_data.next().await {
            if doc.is_ok() == true {
                posts.push(from_document::<Self>(doc.unwrap()).unwrap())
            }
        }
        Ok(posts)
    }

    pub async fn get_unarchived_posts() -> Result<Vec<Post>> {
        let filter = doc! {
            "read": false
        };
        let mut bson_data = Db::find("posts".to_string(), filter, None).await?;
        let mut posts = Vec::new();
        for doc in bson_data.next().await {
            if doc.is_ok() == true {
                posts.push(from_document::<Self>(doc.unwrap()).unwrap())
            }
        }
        Ok(posts)
    }
    pub async fn delete_post(original_url: &str) -> Result<()> {
        let filter = doc! {
            "original_url" : original_url,
        };
        let _ = Db::delete_one("posts".to_string(), filter, None).await?;
        Ok(())
    }
    pub async fn archive_post(original_url: &str) -> Result<()> {
        let filter = doc! {
            "original_url": original_url,
        };
        let update = doc! {
            "read": "true"
        };
        let _ = Db::update("posts".to_string(), filter, update, None).await?;
        Ok(())
    }
}
