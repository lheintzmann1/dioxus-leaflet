use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::{MarkerIcon, PopupOptions};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkerType {
    Pin,
    Circle { radius_px: u32 },
}

/// Represents a marker on the map
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapMarker {
    pub r#type: MarkerType,
    pub lat: f64,
    pub lng: f64,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<MarkerIcon>,
    pub popup_options: Option<PopupOptions>,
    pub custom_data: Option<HashMap<String, String>>,
}

impl MapMarker {
    /// Creates a new MapMarker with basic information
    pub fn new(lat: f64, lng: f64, title: impl Into<String>) -> Self {
        Self {
            lat,
            lng,
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn new_circle(radius_px: u32, lat: f64, lng: f64, title: impl Into<String>) -> Self {
        Self {
            r#type: MarkerType::Circle { radius_px },
            lat,
            lng,
            title: title.into(),
            ..Default::default()
        }
    }

    /// Adds a description to the marker
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a custom icon to the marker
    pub fn with_icon(mut self, icon: MarkerIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Adds custom popup options
    pub fn with_popup_options(mut self, options: PopupOptions) -> Self {
        self.popup_options = Some(options);
        self
    }

    /// Adds custom data
    pub fn with_custom_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.custom_data.is_none() {
            self.custom_data = Some(HashMap::new());
        }
        if let Some(ref mut data) = self.custom_data {
            data.insert(key.into(), value.into());
        }
        self
    }
}

impl Default for MapMarker {
    fn default() -> Self {
        Self {
            r#type: MarkerType::Pin,
            lat: 0.0,
            lng: 0.0,
            title: String::new(),
            description: None,
            icon: None,
            popup_options: None,
            custom_data: None,
        }
    }
}
