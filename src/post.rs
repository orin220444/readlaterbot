use crate::db::db;
use anyhow::Result;
use chrono::prelude::Utc;
use serde_derive::{Deserialize, Serialize};
use serde_rusqlite::*;
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
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

    async fn save_to_db(self) -> Result<()> {
        db::insert_one(
            &self,
            "posts".to_string(),
            "original_url, real_url, read, created".to_string(),
            ":original_url, :real_url, :read, :created".to_string(),
        )?;
        Ok(())
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
        Ok(db::get_all("posts".to_string(), &self)?)
    }

    pub async fn get_unarchived_posts(self) -> Result<Vec<Post>> {
        Ok(db::get_specific(
            "posts".to_string(),
            &self,
            "read = 0".to_string(),
        )?)
    }
    pub async fn delete_post(original_url: &str) -> Result<()> {
        Ok(db::delete(
            "posts".to_string(),
            format!("original_url: {}", original_url),
        )
        .await?)
    }
    pub async fn archive_post(original_url: &str) -> Result<()> {
        Ok(db::update(
            "posts".to_string(),
            "read = 1".to_string(),
            format!("original_url: {}", original_url),
        )
        .await?)
    }
}
