pub mod post {
    use rusqlite::{params, Connection, Result};
    #[derive(Debug)]
    pub struct Post {
        pub original_url: String,
        pub real_url: Option<String>,
        pub read: bool,
    }

    impl Post {
        pub fn new(original_url: String) -> Post {
            Post {
                original_url,
                real_url: None,
                read: false,
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
                "CREATE TABLE IF NOT EXISTS post (
                        original_url    TEXT PRIMARY KEY,
                        real_url        TEXT
                    )",
                params![],
            )?;
            conn.execute(
                "INSERT INTO post (original_url, real_url) VALUES (?1,?2)",
                params![self.original_url, self.real_url],
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
            let conn = Connection::open(&path)?;

            let mut db_data = conn.prepare("SELECT original_url, real_url FROM post")?;
                    ));
            match db_posts {
                Err(e) => {println!("{:#?}", e);
                    Ok(())
                
                },
                Ok(posts) => {
            
            let mut urls = Vec::new();
            for post in posts {
                    println!("{:?}", &post);
                urls.push(post?.original_url);
            }
            println!("{:#?}", urls);
            Ok(())
                }
                }
        }
    }
}
