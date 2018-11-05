use types::*;

/// This object represents one button of the reply keyboard. For simple text buttons String can be used instead of this object to specify text of the button. Optional fields are mutually exclusive.
#[derive(Debug, Serialize, Getters, Deserialize, Clone)]
#[get(vis = "pub")]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    text: String,
    /// If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    request_contact: Option<bool>,
    /// If True, the user's current location will be sent when the button is pressed. Available in private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    request_location: Option<bool>,
}
