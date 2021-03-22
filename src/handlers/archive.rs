use crate::keyboards::unarchive_keyboard;
use crate::Post;
use anyhow::Result;
use teloxide::prelude::*;

pub async fn archive(cx: UpdateWithCx<AutoSend<Bot>,CallbackQuery>, data: &str) -> Result<()> {
    Post::archive_post(data).await?;
    let inline_message_id = cx.update.inline_message_id.clone().unwrap();
    cx.requester
        .answer_callback_query(cx.update.id)
        .text("Archived!")
        .send()
        .await?;
    cx.requester
        .edit_message_text_inline(
            inline_message_id.clone(),
            format!("{} [Archived]", cx.update.message.unwrap().text().unwrap()),
        )
        .send()
        .await?;
    cx.requester
        .edit_message_reply_markup_inline(inline_message_id.clone())
        .reply_markup(unarchive_keyboard(data))
        .send()
        .await?;
    Ok(())
}
