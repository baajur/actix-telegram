use types::*;

/// This object represents a point on the map.
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
}
