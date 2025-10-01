use serde::{Deserialize, Serialize};

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