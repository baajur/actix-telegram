use crate::types::{AllowedUpdate, InputFile, True};
use std::num::NonZeroU8;

/// Use this method to specify a url and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified url, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
/// If you'd like to make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
#[derive(Debug, Serialize, TelegramApi, Setters, New)]
#[return_type = "True"]
#[set(vis = "pub", optional)]
#[new(vis = "pub")]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub(crate) url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[serde(skip_serializing)]
    pub(crate) certificate: Option<InputFile>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_connections: Option<MaxConnections>,
    /// List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) allowed_updates: Option<Vec<AllowedUpdate>>,
}

#[derive(Debug, Serialize, Copy, Clone)]
pub struct MaxConnections(NonZeroU8);

impl From<u8> for MaxConnections {
    #[inline(always)]
    fn from(from: u8) -> Self {
        unsafe { MaxConnections(NonZeroU8::new_unchecked(from)) }
    }
}
