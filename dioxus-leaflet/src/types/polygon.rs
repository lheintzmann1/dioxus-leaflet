use serde::Serialize;
use crate::{LatLng, PathOptions, PopupOptions};

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Polygon {
    pub coordinates: Vec<LatLng>,
    pub path_options: Option<PathOptions>,
    pub title: String,
    pub description: Option<String>,
    pub popup_options: Option<PopupOptions>,
}