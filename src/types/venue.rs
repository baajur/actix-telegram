use super::*;

/// This object represents a venue.
#[derive(Serialize, Deserialize, Debug)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
}