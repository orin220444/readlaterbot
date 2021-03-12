use crate::db::db;
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_rusqlite::*;
use typed_builder::TypedBuilder;
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Post {
    id: i64,
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}

impl Post {
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
            format!("original_url = \"{}\"", original_url),
        )
        .await?)
    }
    pub async fn archive_post(original_url: &str) -> Result<()> {
        Ok(db::update(
            "posts".to_string(),
            "read = 1".to_string(),
            format!("original_url = \"{}\"", original_url),
        )
        .await?)
    }
    pub async fn unarchive_post(original_url: &str) -> Result<()> {
        Ok(db::update(
            "posts".to_string(),
            "read = 0".to_string(),
            format!("original_url = \"{}\"", original_url),
        )
        .await?)
    }
}
#[derive(Serialize, Default, TypedBuilder)]
pub struct PostBuilder {
    pub original_url: String,
    pub real_url: String,
    pub read: bool,
    pub created: String,
}
impl PostBuilder {
    pub async fn save_to_db(&self) -> Result<i64> {
        Ok(db::insert_one(
            &self,
            "posts".to_string(),
            "original_url, real_url, read, created".to_string(),
            ":original_url, :real_url, :read, :created".to_string(),
        )?)
    }
}
