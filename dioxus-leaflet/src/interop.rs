use std::error::Error;
use dioxus::prelude::*;
use dioxus_use_js::JsError;

use crate::{LatLng, MapOptions, MapPosition, MarkerIcon, PathOptions, PopupOptions, types::Id};

pub const DL_JS: Asset = asset!("/assets/dioxus_leaflet.js");

mod js_api {
    use dioxus::prelude::*;
    use dioxus_use_js::use_js;

    use_js!("js_utils/src/map.ts", "assets/dioxus_leaflet.js"::{update_map, delete_map, on_map_click});
    use_js!("js_utils/src/marker.ts", "assets/dioxus_leaflet.js"::{update_marker, delete_marker});
    use_js!("js_utils/src/polygon.ts", "assets/dioxus_leaflet.js"::{update_polygon, delete_polygon});
    use_js!("js_utils/src/popup.ts", "assets/dioxus_leaflet.js"::{update_popup});
}

fn js_to_eval(err: JsError) -> Box<dyn Error + Send + Sync> {
    match err {
        JsError::Eval(err) => err.into(),
        _ => panic!("Unexpected JsError variant"),
    }
}

pub async fn update_map<'a>(
    id: &Id, initial_position: &MapPosition, options: &MapOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_map(id, initial_position, options).await.map_err(js_to_eval)
}

pub async fn delete_map<'a>(id: &Id) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_map(id).await.map_err(js_to_eval)
}

pub async fn on_map_click(
    map_id: &Id,
    event_handler: EventHandler<LatLng>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::on_map_click(map_id, async |coords| {
        event_handler(LatLng::new(coords[0], coords[1]));
        Ok(())
    }).await.map_err(|e| e.into())
}

pub async fn update_marker(
    marker_id: &Id,
    coordinate: &LatLng,
    icon: &Option<MarkerIcon>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_marker(marker_id.parent().unwrap(), marker_id.id(), coordinate, icon).await.map_err(js_to_eval)
}

pub async fn delete_marker(marker_id: &Id) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_marker(marker_id.parent().unwrap(), marker_id.id()).await.map_err(js_to_eval)
}

pub async fn update_polygon(
    polygon_id: &Id,
    coordinates: &Vec<Vec<Vec<LatLng>>>,
    options: &PathOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_polygon(polygon_id.parent().unwrap(), polygon_id.id(), coordinates, options).await.map_err(js_to_eval)
}

pub async fn delete_polygon(polygon_id: &Id) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_polygon(polygon_id.parent().unwrap(), polygon_id.id()).await.map_err(js_to_eval)
}

pub async fn update_popup(
    popup_id: &Id,
    options: &PopupOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let marker_id = popup_id.parent().unwrap();
    js_api::update_popup(marker_id, popup_id, options).await.map_err(js_to_eval)
}