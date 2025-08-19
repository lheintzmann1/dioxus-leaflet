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
    pub zoom_control: bool,
    pub scroll_wheel_zoom: bool,
    pub double_click_zoom: bool,
    pub touch_zoom: bool,
    pub dragging: bool,
    pub keyboard: bool,
    pub attribution_control: bool,
    pub tile_layer: TileLayer,
    pub leaflet_resources: LeafletResources,
}

impl Default for MapOptions {
    fn default() -> Self {
        Self {
            zoom_control: true,
            scroll_wheel_zoom: true,
            double_click_zoom: true,
            touch_zoom: true,
            dragging: true,
            keyboard: true,
            attribution_control: true,
            tile_layer: TileLayer::default(),
            leaflet_resources: LeafletResources::default(),
        }
    }
}

impl MapOptions {
    /// Creates a MapOptions with all controls disabled and default tile layer
    pub fn minimal() -> Self {
        Self {
            zoom_control: false,
            scroll_wheel_zoom: false,
            double_click_zoom: false,
            touch_zoom: false,
            dragging: false,
            keyboard: false,
            attribution_control: false,
            tile_layer: TileLayer::default(),
            leaflet_resources: LeafletResources::default(),
        }
    }

    /// Builder method to enable/disable zoom control
    pub fn with_zoom_control(mut self, enabled: bool) -> Self {
        self.zoom_control = enabled;
        self
    }

    /// Builder method to enable/disable scroll wheel zoom
    pub fn with_scroll_wheel_zoom(mut self, enabled: bool) -> Self {
        self.scroll_wheel_zoom = enabled;
        self
    }

    /// Builder method to enable/disable double click zoom
    pub fn with_double_click_zoom(mut self, enabled: bool) -> Self {
        self.double_click_zoom = enabled;
        self
    }

    /// Builder method to enable/disable touch zoom
    pub fn with_touch_zoom(mut self, enabled: bool) -> Self {
        self.touch_zoom = enabled;
        self
    }

    /// Builder method to enable/disable dragging
    pub fn with_dragging(mut self, enabled: bool) -> Self {
        self.dragging = enabled;
        self
    }

    /// Builder method to enable/disable keyboard controls
    pub fn with_keyboard(mut self, enabled: bool) -> Self {
        self.keyboard = enabled;
        self
    }

    /// Builder method to enable/disable attribution control
    pub fn with_attribution_control(mut self, enabled: bool) -> Self {
        self.attribution_control = enabled;
        self
    }

    /// Builder method to set tile layer
    pub fn with_tile_layer(mut self, tile_layer: TileLayer) -> Self {
        self.tile_layer = tile_layer;
        self
    }

    /// Builder method to set Leaflet resources configuration
    pub fn with_leaflet_resources(mut self, resources: LeafletResources) -> Self {
        self.leaflet_resources = resources;
        self
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

/// Leaflet resource configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LeafletResources {
    /// Use CDN with specified version
    Cdn {
        version: String,
        base_url: Option<String>, // Allow custom CDN base URL
    },
    /// Use local files
    Local {
        css_path: String,
        js_path: String,
    },
}

impl LeafletResources {
    /// Creates a CDN configuration with the specified version
    pub fn cdn(version: impl Into<String>) -> Self {
        Self::Cdn {
            version: version.into(),
            base_url: None,
        }
    }

    /// Creates a CDN configuration with custom base URL
    pub fn cdn_with_base_url(version: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self::Cdn {
            version: version.into(),
            base_url: Some(base_url.into()),
        }
    }

    /// Creates a local files configuration
    pub fn local(css_path: impl Into<String>, js_path: impl Into<String>) -> Self {
        Self::Local {
            css_path: css_path.into(),
            js_path: js_path.into(),
        }
    }

    /// Returns the CSS URL/path
    pub fn css_url(&self) -> String {
        match self {
            Self::Cdn { version, base_url } => {
                let base = base_url.as_deref().unwrap_or("https://unpkg.com");
                format!("{}/leaflet@{}/dist/leaflet.css", base, version)
            }
            Self::Local { css_path, .. } => css_path.clone(),
        }
    }

    /// Returns the JS URL/path
    pub fn js_url(&self) -> String {
        match self {
            Self::Cdn { version, base_url } => {
                let base = base_url.as_deref().unwrap_or("https://unpkg.com");
                format!("{}/leaflet@{}/dist/leaflet.js", base, version)
            }
            Self::Local { js_path, .. } => js_path.clone(),
        }
    }

    /// Returns the integrity hash for CSS (if using CDN with known versions)
    pub fn css_integrity(&self) -> Option<String> {
        match self {
            Self::Cdn { version, base_url } if base_url.is_none() => {
                // Only provide integrity for unpkg.com with known versions
                match version.as_str() {
                    "1.9.4" => Some("sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=".to_string()),
                    "1.9.3" => Some("sha256-kLaT2GOSpHechhsozzB+flnD+zUyjE2LlfWPgU04xyI=".to_string()),
                    "1.9.2" => Some("sha256-sA+zWATbFveLLNqWO2gtiw3HL/lh1giY/Inf1BJ0z14=".to_string()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Returns the integrity hash for JS (if using CDN with known versions)
    pub fn js_integrity(&self) -> Option<String> {
        match self {
            Self::Cdn { version, base_url } if base_url.is_none() => {
                // Only provide integrity for unpkg.com with known versions
                match version.as_str() {
                    "1.9.4" => Some("sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=".to_string()),
                    "1.9.3" => Some("sha256-WBkoXOwTeyKclOHuWtc+i2uENFpDZ9YPdf5Hf+D7ewM=".to_string()),
                    "1.9.2" => Some("sha256-o9N4PsYA2zOcVD5OHEHviWpTGQ4Q1jEzU7oJiE+zRCE=".to_string()),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

impl Default for LeafletResources {
    fn default() -> Self {
        Self::cdn("1.9.4")
    }
}
