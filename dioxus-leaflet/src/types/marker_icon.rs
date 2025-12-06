use serde::{Serialize, Deserialize};

/// Custom marker icon configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
