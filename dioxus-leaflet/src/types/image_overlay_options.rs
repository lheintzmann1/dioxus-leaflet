use serde::{Deserialize, Serialize};

///
/// All options for the ImageOverlay
/// As of 1.8 only opacity is supported
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOverlayOptions {
    pub opacity: Option<f64>,
    pub alt: Option<String>,
    pub interactive: Option<bool>,
    pub cross_origin: Option<bool>,
    pub error_overlay_url: Option<String>,
    pub z_index: Option<i32>,
    pub class_name: Option<String>,
    pub decoding: Option<String>,
}

impl Default for ImageOverlayOptions {
    fn default() -> Self {
        Self {
            opacity: Some(1.0),
            alt: Some(String::new()),
            interactive: Some(false),
            cross_origin: Some(false),
            error_overlay_url: Some(String::new()),
            z_index: Some(1),
            class_name: Some(String::new()),
            decoding: Some(String::from("auto")),
        }
    }
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
