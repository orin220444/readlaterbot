use teloxide::types::{MediaKind, Message, MessageEntityKind, MessageKind};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButtonKind, InlineKeyboardMarkup, MessageEntity},
};

pub fn link_finder(message: &UpdateWithCx<AutoSend<Bot>, Message>) -> Vec<String> {
    let message_kind = &message.update.kind;
    let mut urls: Vec<String> = Vec::new();
    parse_media_kind(message_kind, &mut urls);
    if let Some(keyboard) = message.update.reply_markup() {
        parse_keyboard(keyboard, &mut urls);
    }
    urls
}
fn entities_parse(entity: &MessageEntity, text: &String) -> Option<String> {
    let mut res = None;
    match &entity.kind {
        MessageEntityKind::TextLink { url } => res = Some(url.to_string()),
        MessageEntityKind::Url => {
            let url: String = {
                let text = text;
                let start = entity.offset;
                let length = entity.length;
                text.chars().skip(start).take(length).collect()
            };
            println!("{:#?}", url);
            res = Some(url);
        }
        _ => println!("No needed entities!"),
    };
    res
}
fn parse_media_kind(message_kind: &MessageKind, urls: &mut Vec<String>) {
    match message_kind {
        MessageKind::Common(message_common) => match &message_common.media_kind {
            MediaKind::Text(text_data) => {
                //let mut urls: Vec<String> = Vec::new();
                for entity in &text_data.entities {
                    let text = &text_data.text;
                    if let Some(url) = entities_parse(entity, text) {
                        urls.push(url);
                    }
                }
                Some(urls);
            }
            MediaKind::Photo(data) => {
                //let mut urls:Vec<String> = Vec::new();
                for entity in &data.caption_entities {
                    let caption = match &data.caption {
                        Some(str) => str.to_string(),
                        None => String::default(),
                    };
                    println!("{:#?}", caption);
                    if let Some(url) = entities_parse(entity, &caption) {
                        urls.push(url);
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}
fn parse_keyboard(keyboard: &InlineKeyboardMarkup, urls: &mut Vec<String>) {
    for row in &keyboard.inline_keyboard {
        for button in row {
            match &button.kind {
                InlineKeyboardButtonKind::Url(url) => urls.push(url.to_string()),
                _ => {}
            }
        }
    }
}
