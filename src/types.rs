use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Represents a marker on the map
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapMarker {
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
            description: None,
            icon: None,
            popup_options: None,
            custom_data: None,
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

/// Custom marker icon configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerIcon {
    pub icon_url: String,
    pub icon_size: Option<(u32, u32)>,
    pub icon_anchor: Option<(u32, u32)>,
    pub popup_anchor: Option<(i32, i32)>,
    pub shadow_url: Option<String>,
    pub shadow_size: Option<(u32, u32)>,
}

impl MarkerIcon {
    /// Creates a new MarkerIcon with just the URL
    pub fn new(icon_url: impl Into<String>) -> Self {
        Self {
            icon_url: icon_url.into(),
            icon_size: None,
            icon_anchor: None,
            popup_anchor: None,
            shadow_url: None,
            shadow_size: None,
        }
    }
}

/// Popup configuration options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopupOptions {
    pub max_width: Option<u32>,
    pub min_width: Option<u32>,
    pub max_height: Option<u32>,
    pub auto_pan: Option<bool>,
    pub keep_in_view: Option<bool>,
    pub close_button: Option<bool>,
    pub auto_close: Option<bool>,
    pub close_on_escape_key: Option<bool>,
    pub class_name: Option<String>,
}

impl Default for PopupOptions {
    fn default() -> Self {
        Self {
            max_width: Some(300),
            min_width: Some(50),
            max_height: None,
            auto_pan: Some(true),
            keep_in_view: Some(false),
            close_button: Some(true),
            auto_close: Some(true),
            close_on_escape_key: Some(true),
            class_name: None,
        }
    }
}

/// Map configuration options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapOptions {
    pub zoom_control: Option<bool>,
    pub scroll_wheel_zoom: Option<bool>,
    pub double_click_zoom: Option<bool>,
    pub touch_zoom: Option<bool>,
    pub dragging: Option<bool>,
    pub keyboard: Option<bool>,
    pub attribution_control: Option<bool>,
    pub tile_layer: Option<TileLayer>,
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            zoom_control: Some(true),
            scroll_wheel_zoom: Some(true),
            double_click_zoom: Some(true),
            touch_zoom: Some(true),
            dragging: Some(true),
            keyboard: Some(true),
            attribution_control: Some(true),
            tile_layer: Some(TileLayer::default()),
        }
    }
}

impl MapOptions {
    /// Creates a new MapOptions with all fields set to None (minimal configuration)
    pub fn new() -> Self {
        Self {
            zoom_control: None,
            scroll_wheel_zoom: None,
            double_click_zoom: None,
            touch_zoom: None,
            dragging: None,
            keyboard: None,
            attribution_control: None,
            tile_layer: None,
        }
    }

    /// Gets the zoom control setting, falling back to default if None
    pub fn zoom_control(&self) -> bool {
        self.zoom_control.unwrap_or(true)
    }

    /// Gets the scroll wheel zoom setting, falling back to default if None
    pub fn scroll_wheel_zoom(&self) -> bool {
        self.scroll_wheel_zoom.unwrap_or(true)
    }

    /// Gets the double click zoom setting, falling back to default if None
    pub fn double_click_zoom(&self) -> bool {
        self.double_click_zoom.unwrap_or(true)
    }

    /// Gets the touch zoom setting, falling back to default if None
    pub fn touch_zoom(&self) -> bool {
        self.touch_zoom.unwrap_or(true)
    }

    /// Gets the dragging setting, falling back to default if None
    pub fn dragging(&self) -> bool {
        self.dragging.unwrap_or(true)
    }

    /// Gets the keyboard setting, falling back to default if None
    pub fn keyboard(&self) -> bool {
        self.keyboard.unwrap_or(true)
    }

    /// Gets the attribution control setting, falling back to default if None
    pub fn attribution_control(&self) -> bool {
        self.attribution_control.unwrap_or(true)
    }

    /// Gets the tile layer, falling back to default if None
    pub fn tile_layer(&self) -> TileLayer {
        self.tile_layer.clone().unwrap_or_default()
    }
}

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
