use crate::types::*;

/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Debug, Deserialize, Clone, Getters)]
#[get(vis = "pub")]
pub struct Game {
    /// Title of the game
    pub(crate) title: String,
    /// Description of the game
    pub(crate) description: String,
    /// Photo that will be displayed in the game message in chats.
    pub(crate) photo: Vec<PhotoSize>,
    /// Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls setGameScore, or manually edited using editMessageText. 0-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text: Option<String>,
    /// Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text_entities: Option<Vec<MessageEntity>>,
    /// Animation that will be displayed in the game message in chats. Upload via BotFather
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) animation: Option<Animation>,
}