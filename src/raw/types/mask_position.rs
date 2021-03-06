use crate::types::*;

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Debug, Deserialize, Clone, Getters, Serialize, Setters, New)]
#[get(vis = "pub")]
#[new(vis = "pub")]
#[set(vis = "pub")]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub(crate) point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub(crate) x_shift: Float,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub(crate) y_shift: Float,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub(crate) scale: Float,
}