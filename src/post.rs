use chrono::prelude::{DateTime, Utc};
use rusqlite::{params, Connection, Result, NO_PARAMS};
use serde_derive::Deserialize;
use serde_rusqlite::*;
#[derive(Deserialize, Debug, PartialEq)]
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
        let path = "./readlaterdb.db3";
        let conn = Connection::open(&path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS posts (
                        original_url    TEXT PRIMARY KEY,
                        real_url        TEXT,
                        read            BIT,
                        created         TEXT
                    )",
            NO_PARAMS,
        )?;
        conn.execute(
            "INSERT INTO posts (original_url, real_url, read, created) VALUES (?1, ?2, ?3, ?4)",
            params![
                &self.original_url,
                &self.real_url,
                &self.read,
                &self.created
            ],
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
                //let body = &res.text().await?;
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
    pub async fn get_all_posts() -> Result<Vec<Post>> {
        let path = "./readlaterdb.db3";
        let conn = Connection::open(&path).unwrap();

        let mut db_data = conn
            .prepare("SELECT original_url, real_url, read FROM post")
            .unwrap();
        let mut res = from_rows::<Post>(db_data.query(NO_PARAMS).unwrap());
        let mut posts = Vec::new();
        for post in res {
            match post {
                Ok(post) => posts.push(post),
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(posts)
    }

    pub async fn get_unarchived_posts() -> Result<Vec<Post>> {
        let path = "./readlaterdb.db3";
        let conn = Connection::open(&path)?;

        let mut db_data =
            conn.prepare("SELECT original_url, real_url, read FROM post WHERE read=0")?;
        let mut res = from_rows::<Post>(db_data.query(NO_PARAMS).unwrap());
        let mut posts = Vec::new();
        for post in res {
            match post {
                Ok(post) => posts.push(post),
                Err(e) => println!("{:#?}", e),
            }
        }
        Ok(posts)
    }
    pub async fn delete_post(original_url: &str) -> Result<()> {
        let path = "./readlaterdb.db3";
        let conn = Connection::open(&path)?;
        conn.execute(
            "DELETE FROM post WHERE original_url=?1",
            params![original_url],
        )?;
        Ok(())
    }
    pub async fn archive_post(original_url: &str) -> Result<()> {
        let path = "./readlaterdb.db3";
        let conn = Connection::open(&path)?;
        conn.execute(
            "UPDATE post SET read = 1 WHERE original_url=?1",
            params![original_url],
        )?;
        Ok(())
    }
}
