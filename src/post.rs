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
    id: String,
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}

impl Post {
    pub async fn get_all_posts(self) -> Result<Vec<Post>> {
        let filter = doc! {};
        let mut bson_data = Db::find("posts", filter, None).await?;
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
        let mut bson_data = Db::find("posts", filter, None).await?;
        let mut posts = Vec::new();
        for doc in bson_data.next().await {
            if doc.is_ok() == true {
                posts.push(from_document::<Self>(doc.unwrap()).unwrap())
            }
        }
        Ok(posts)
    }
    pub async fn delete_post(id: &str) -> Result<()> {
        let filter = doc! {
            "_id" : id,
        };
        let _ = Db::delete_one("posts", filter, None).await?;
        Ok(())
    }
    pub async fn archive_post(id: &str) -> Result<()> {
        let filter = doc! {
            "_id": id,
        };
        let update = doc! {
            "read": "true"
        };
        let _ = Db::update("posts", filter, update, None).await?;
        Ok(())
    }
    pub async fn unarchive_post(id: &str) -> Result<()> {
        let filter = doc! {
            "_id": id,
        };
        let update = doc! {
            "read": "false"
        };
        let _ = Db::update("posts", filter, update, None).await?;
        Ok(())
    }
    pub fn id(&self) -> String {
        self.id.clone()
    }
}
#[derive(TypedBuilder, Serialize)]
pub struct PostBuilder {
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}
impl PostBuilder {
    pub async fn save_to_db(&self) -> Result<String> {
        let bson_post = to_document(&self)?;
        println!("{:#?}", bson_post);
        let bson_res = Db::insert_one("posts", bson_post, None).await?;
        println!("{:#?}", bson_res);
        let id_str = bson_res.inserted_id.as_object_id().unwrap().to_string();
        Ok(id_str)
    }
}
