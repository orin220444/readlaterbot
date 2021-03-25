use crate::keyboards;
use crate::link_finder;
use crate::post::{Post, PostBuilder};
use anyhow::Result;
use teloxide::prelude::*;
pub async fn add(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> Result<()> {
    println!("{:#?}", &cx.update.kind);
    let urls = link_finder::link_finder(&cx);
    if urls.is_empty() {
        println!("No urls!");
    }
    println!("{:#?}", urls);
    for url in urls {
        let real_url = real_url(&url).await;
        let created = created();
        let mut post = PostBuilder::builder()
            .original_url(url.clone())
            .real_url(real_url)
            .created(created)
            .read(false)
            .build();
        match post.save_to_db().await {
            Ok(id) => {
                log::info!("Successful saved post");
                match cx
                    .answer(url)
                    .reply_markup(keyboards::standart_keyboard(&format!("{}", id)))
                    .send()
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error while sending linkfinder url! {:#?}", e)
                    }
                }
            }
            Err(e) => println!("{:#?}", e),
        }
    }

    Ok(())
}
use url::Url;
async fn real_url(original_url: &str) -> String {
    match reqwest::get(original_url).await {
        Err(e) => {
            println!("{:#?}", e);
            original_url.into()
        }
        Ok(res) => {
            println!("{:#?}", &res);
            let mut real_url = {
                let mut host: String = String::new();
                match res.url().host() {
                    Some(host_path) => host = host_path.to_string(),
                    None => {
                        println!("No host!");
                    }
                }
                let path = res.url().path().to_string();
                if let Some(query) = res.url().query() {
                    host + &path + "?" + query
                } else {
                    host + &path
                }
            };
            println!("{}", real_url);
            if let Err(_) = Url::parse(&real_url) {
                real_url = original_url.into();
            }
            real_url
        }
    }
}
use chrono::prelude::Utc;
fn created() -> String {
    Utc::now().to_string()
}
