use crate::types::*;

/// Use this method to send invoices. On success, the sent Message is returned.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "Message"]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct SendInvoice {
    /// Unique identifier for the target private chat
    pub(crate) chat_id: Integer,
    /// Product name, 1-32 characters
    pub(crate) title: String,
    /// Product description, 1-255 characters
    pub(crate) description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub(crate) payload: String,
    /// Payments provider token, obtained via Botfather
    pub(crate) provider_token: String,
    /// Unique deep-linking parameter that can be used to generate this invoice when used as a start parameter
    pub(crate) start_parameter: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub(crate) currency: String,
    /// Price breakdown, a list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub(crate) prices: Vec<LabeledPrice>,
    /// JSON-encoded data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo_url: Option<String>,
    /// Photo size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo_size: Option<Integer>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo_width: Option<Integer>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) photo_height: Option<Integer>,
    /// Pass True, if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) need_name: Option<bool>,
    /// Pass True, if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) need_phone_number: Option<bool>,
    /// Pass True, if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) need_email: Option<bool>,
    /// Pass True, if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) need_shipping_address: Option<bool>,
    /// Pass True, if user's phone number should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) send_phone_number_to_provider: Option<bool>,
    /// Pass True, if user's email address should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) send_email_to_provider: Option<bool>,
    /// Pass True, if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_flexible: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_to_message_id: Option<Integer>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_markup: Option<InlineKeyboardMarkup>,
}