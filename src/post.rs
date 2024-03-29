use crate::db::db;
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

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
        Ok(db::get_all("posts", &self)?)
    }

    pub async fn get_unarchived_posts(self) -> Result<Vec<Post>> {
        Ok(db::get_specific("posts", &self, "read = 0")?)
    }
    pub async fn delete_post(id: &str) -> Result<()> {
        Ok(db::delete("posts", &format!(r#"id = {}"#, id)).await?)
    }
    pub async fn archive_post(id: &str) -> Result<()> {
        Ok(db::update("posts", "read = 1", &format!(r#"id = {}"#, id)).await?)
    }
    pub async fn unarchive_post(id: &str) -> Result<()> {
        Ok(db::update("posts", "read = 0", &format!(r#"id = {}"#, id)).await?)
    }
    pub fn id(&self) -> i64 {
        self.id
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
            "posts",
            "original_url, real_url, read, created",
            ":original_url, :real_url, :read, :created",
        )?)
    }
}
