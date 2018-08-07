use super::*;

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be gif
    #[serde(rename = "type")]
    pub type_: String,
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Width of the GIF
    pub gif_width: Option<Integer>,
    /// Height of the GIF
    pub gif_height: Option<Integer>,
    /// Duration of the GIF
    pub gif_duration: Option<Integer>,
    /// URL of the static thumbnail for the result (jpeg or gif)
    pub thumb_url: String,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-200 characters
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    pub parse_mode: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}