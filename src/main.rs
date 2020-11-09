use teloxide::prelude::*;
#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        println!("{:#?}", &message.update.kind);
        let urls = link_finder::link_finder(&message);
match urls {
    Some(urls) => {
        println!("{:#?}", urls);
        for url in urls{
            post::Post::save_post(url).await;
        }
    },
    None => println!("No urls!")
}
        message.answer_dice().send().await?;
        ResponseResult::<()>::Ok(())
    })
    .await;
}
mod link_finder {
    use teloxide::dispatching::UpdateWithCx;
    use teloxide::types::{MessageKind, Message, MediaKind, MessageEntityKind};

    pub fn link_finder(message: &UpdateWithCx<Message>)-> Option<Vec<String>>{
        let message_kind = &message.update.kind;
        match message_kind {
            MessageKind::Common(message_common) => {
                println!("{:#?}", message_common.media_kind);
                match &message_common.media_kind {
                    MediaKind::Text(text_data) => {
                        let mut urls:Vec<String> = Vec::new();
                        println!("{:#?}", &text_data);
                        for entity in &text_data.entities {
                            println!("{:#?}", entity);
                            match &entity.kind {
                                MessageEntityKind::TextLink { url} => {
                                    urls.push(url.to_string())
                                },
                                MessageEntityKind::Url => { println!("{:#?}", &entity)}
                                _ => println!("No needed entities!")
                            };
                        }
                        return Some(urls)
                    },
                    _=> return None,

                }
            },
            _ => return None
        }
    }
}
pub mod post{
use reqwest::get;
pub struct Post {
    originalUrl:String,
    realUrl: String,
}
impl Post {
    pub async fn save_post(url:String) -> Result<(), reqwest::Error> {
        let res = reqwest::get(&url.to_string()).await?;
        //let body = &res.text().await?;
        println!("{:#?}", &res);
        let mut host:String = String::new();
        match res.url().host() {
            Some(host_path) => {host = host_path.to_string()}
            None => { println!("No host!");
            }}

        let path = res.url().path();
        let real_url = host + path;
        println!("{}", real_url);
        Ok(())
    }

}
}
