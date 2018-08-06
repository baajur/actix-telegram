use super::super::types::*;

/// Use this method to pin a message in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the ‘can_pin_messages’ admin right in the supergroup or ‘can_edit_messages’ admin right in the channel. Returns True on success.
#[derive(Serialize, Deserialize, Debug)]
pub struct PinChatMessage {
    chat_id: ChatId,
    message_id: Integer,
    disable_notification: Option<bool>,
}