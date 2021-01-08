use crate::db::Db;
use anyhow::Result;
use chrono::prelude::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use futures::stream::StreamExt;
use mongodb::bson::{doc, ser::to_document, de::from_document};
use mongodb::bson::Document;
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Post {
    pub original_url: String,
    pub real_url: Option<String>,
    pub read: bool,
    pub created: String,
}

impl Post {
    pub fn new(original_url: &str) -> Post {
        Post {
            original_url: original_url.to_string(),
            real_url: None,
            read: false,
            created: Utc::now().to_string(),
        }
    }

    pub async fn save_post(self) -> Result<()> {
        self.save_to_db().await?;
        Ok(())
    }

    async fn save_to_db(&self) -> Result<()> {
        let bson_post = to_document(&self)?;
        Ok(Db::insert_one("posts".to_string(), bson_post, None).await?)
    }
    pub async fn real_url(&mut self) -> &Post {
        match reqwest::get(&self.original_url.to_string()).await {
            Err(e) => {
                println!("{:#?}", e);
                self
            }
            Ok(res) => {
                println!("{:#?}", &res);
                let real_url = {
                    let mut host: String = String::new();
                    match res.url().host() {
                        Some(host_path) => host = host_path.to_string(),
                        None => {
                            println!("No host!");
                        }
                    }
                    let path = res.url().path().to_string();
                    host + &path
                };
                println!("{}", real_url);

                self.real_url = Some(real_url);
                self
            }
        }
    }
    pub async fn get_all_posts(self) -> Result<Vec<Post>> {
        let filter = doc!{
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

    pub async fn get_unarchived_posts() -> Result<Vec<Post>> {
        let filter = doc!{
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
        let filter = doc!{
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
