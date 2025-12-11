use serde::{Deserialize, Serialize};
use crate::LatLng;

/// Represents a geographical position with zoom level
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapPosition {
    pub coordinates: LatLng,
    pub zoom: f64,
}

impl MapPosition {
    /// Creates a new MapPosition
    pub const fn new(lat: f64, lng: f64, zoom: f64) -> Self {
        Self { coordinates: LatLng::new(lat, lng), zoom }
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        Self::new(51.505, -0.09, 13.0)
    }
}
