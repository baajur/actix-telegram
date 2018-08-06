use super::*;

/// This object represents a Telegram user or bot.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: Integer,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}