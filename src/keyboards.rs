use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup,
};
pub fn standart_keyboard(id: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::new(
            "Delete",
            InlineKeyboardButtonKind::CallbackData(format!("del {}", id).into()),
        ),
        InlineKeyboardButton::new(
            "Archive",
            InlineKeyboardButtonKind::CallbackData(format!("archive {}", id).into()),
        ),
    ]])
}
pub fn unarchive_keyboard(id: &str) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![vec![
        InlineKeyboardButton::new(
            "Delete",
            InlineKeyboardButtonKind::CallbackData(format!("del {}", id).into()),
        ),
        InlineKeyboardButton::new(
            "Unarchive",
            InlineKeyboardButtonKind::CallbackData(format!("unarchive {}", id).into()),
        ),
    ]])
}
