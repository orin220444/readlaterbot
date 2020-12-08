use teloxide::
    types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup};
pub fn standart_keyboard() -> ReplyMarkup{
ReplyMarkup::InlineKeyboardMarkup(
                            InlineKeyboardMarkup::new(vec![vec![
                                InlineKeyboardButton::new(
                                    "Delete",
                                    InlineKeyboardButtonKind::CallbackData("del".into()),
                                ),
                                InlineKeyboardButton::new(
                                    "Archive",
                                    InlineKeyboardButtonKind::CallbackData("archive".into()),
                                ),
                            ]]),
                        )
}
