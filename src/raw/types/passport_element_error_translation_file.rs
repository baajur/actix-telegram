use crate::types::*;

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(Debug, Serialize, Setters, New)]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be translation_file
    pub(crate) source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub(crate) type_: String,
    /// Base64-encoded file hash
    pub(crate) file_hash: String,
    /// Error message
    pub(crate) message: String,
}