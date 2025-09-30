use serde::{Deserialize, Serialize};

/// Represents a geographical position with zoom level
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapPosition {
    pub lat: f64,
    pub lng: f64,
    pub zoom: f64,
}

impl MapPosition {
    /// Creates a new MapPosition
    pub fn new(lat: f64, lng: f64, zoom: f64) -> Self {
        Self { lat, lng, zoom }
    }
}

impl Default for MapPosition {
    fn default() -> Self {
        Self::new(51.505, -0.09, 13.0)
    }
}
