use super::super::types::*;

/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendGame {
    chat_id: Integer,
    game_short_name: String,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<Integer>,
    reply_markup: Option<InlineKeyboardMarkup>,
}