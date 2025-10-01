use serde::{Deserialize, Serialize};
use super::{TileLayer, LeafletResources};

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
