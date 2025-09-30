use serde::{Deserialize, Serialize};

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
