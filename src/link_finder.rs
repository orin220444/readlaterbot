use teloxide::dispatching::UpdateWithCx;
use teloxide::types::{MediaKind, Message, MessageEntityKind, MessageKind};

pub fn link_finder(message: &UpdateWithCx<Message>) -> Option<Vec<String>> {
    let message_kind = &message.update.kind;
    match message_kind {
        MessageKind::Common(message_common) => match &message_common.media_kind {
            MediaKind::Text(text_data) => {
                let mut urls: Vec<String> = Vec::new();
                for entity in &text_data.entities {
                    match &entity.kind {
                        MessageEntityKind::TextLink { url } => urls.push(url.to_string()),
                        MessageEntityKind::Url => {
                            let url: String = {
                                let text = &text_data.text;
                                let start = entity.offset;
                                let length = entity.length;
                                text.chars().skip(start).take(length).collect()
                            };
                            println!("{:#?}", url);
                            urls.push(url.to_string());
                        }
                        _ => println!("No needed entities!"),
                    };
                }
                Some(urls)
            }
            _ => None,
        },
        _ => None,
    }
}
