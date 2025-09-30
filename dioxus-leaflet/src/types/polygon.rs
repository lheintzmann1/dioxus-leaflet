use serde::Serialize;
use crate::{MapPosition, PathOptions, PopupOptions};

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Polygon {
    pub points: Vec<MapPosition>,
    pub path_options: Option<PathOptions>,
    pub title: String,
    pub description: Option<String>,
    pub popup_options: Option<PopupOptions>,
}