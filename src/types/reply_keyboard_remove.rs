use super::*;

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: True,
    selective: Option<bool>,
}