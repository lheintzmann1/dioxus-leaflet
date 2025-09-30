use serde::{Deserialize, Serialize};

/// Tile layer configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TileLayer {
    pub url: String,
    pub attribution: String,
    pub max_zoom: u8,
    pub subdomains: Vec<String>,
}

impl TileLayer {
    /// OpenStreetMap tile layer (default)
    pub fn openstreetmap() -> Self {
        Self {
            url: "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png".to_string(),
            attribution: "&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors".to_string(),
            max_zoom: 19,
            subdomains: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        }
    }

    /// Satellite imagery tile layer
    pub fn satellite() -> Self {
        Self {
            url: "https://server.arcgisonline.com/ArcGIS/rest/services/World_Imagery/MapServer/tile/{z}/{y}/{x}".to_string(),
            attribution: "Tiles &copy; Esri".to_string(),
            max_zoom: 18,
            subdomains: vec![],
        }
    }
}

impl Default for TileLayer {
    fn default() -> Self {
        Self::openstreetmap()
    }
}
