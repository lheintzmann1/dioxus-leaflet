use serde::{Deserialize, Serialize};

/// Options for an [`ImageOverlay`](crate::ImageOverlay).
///
/// Every field is optional: unset (`None`) fields are not sent to Leaflet, which then
/// applies its own default. See <https://leafletjs.com/reference.html#imageoverlay>.
///
/// `opacity` and `z_index` are applied live when they change; the other options only
/// take effect when the overlay is first created.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOverlayOptions {
    /// Opacity of the image, between 0 and 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// Text for the `alt` attribute of the image (accessibility).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    /// Whether the overlay handles pointer events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    /// Whether the `crossOrigin` attribute is set on the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_origin: Option<bool>,
    /// URL of an image to show if the main image fails to load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_overlay_url: Option<String>,
    /// The explicit z-index of the overlay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i32>,
    /// A custom CSS class name for the image element.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    /// The `decoding` hint for the image (`"sync"`, `"async"` or `"auto"`).
    /// Requires Leaflet 1.9 or later.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoding: Option<String>,
}

impl ImageOverlayOptions {
    /// Set the opacity of the image, between 0 and 1.
    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    /// Description for the overlayed image for accessibility use
    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    /// Have the overlay emit pointer events
    pub fn with_interactive(mut self, interactive: bool) -> Self {
        self.interactive = Some(interactive);
        self
    }

    /// Enable cross origin checks.
    pub fn with_cross_origin(mut self, cross_origin: bool) -> Self {
        self.cross_origin = Some(cross_origin);
        self
    }

    /// Set a fallback url when the image fails to load.
    /// In practice, not all browsers properly support the event handling.
    pub fn with_error_overlay_url(mut self, error_overlay_url: impl Into<String>) -> Self {
        self.error_overlay_url = Some(error_overlay_url.into());
        self
    }

    /// Set the z-index of the overlayed image.
    pub fn with_z_index(mut self, z_index: i32) -> Self {
        self.z_index = Some(z_index);
        self
    }

    /// Add a classname to the overlayed image.
    pub fn with_class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = Some(class_name.into());
        self
    }

    /// Specify which decoding method should be used to load the image.
    pub fn with_decoding(mut self, decoding: impl Into<String>) -> Self {
        self.decoding = Some(decoding.into());
        self
    }
}
