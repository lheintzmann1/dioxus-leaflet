use std::error::Error;
use dioxus::prelude::*;
use dioxus_use_js::JsError;
use serde::Serialize;

use crate::{LatLng, MapOptions, MapPosition, MarkerIcon, PathOptions, PopupOptions, components::MapId, components::MarkerId};

pub const DL_JS: Asset = asset!("/assets/dioxus_leaflet.js");

mod js_api {
    use dioxus::prelude::*;
    use dioxus_use_js::use_js;

    use_js!("js_utils/src/map.ts", "assets/dioxus_leaflet.js"::{update_map, delete_map});
    use_js!("js_utils/src/marker.ts", "assets/dioxus_leaflet.js"::{update_marker, delete_marker});
    use_js!("js_utils/src/popup.ts", "assets/dioxus_leaflet.js"::{update_popup});
}

const CALL_REGISTER_ONCLICK_HANDLER_MAP_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.registerOnClickHandlerMapAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

const CALL_POLYGON_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.updatePolygonAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

fn js_to_eval(err: JsError) -> Box<dyn Error + Send + Sync> {
    match err {
        JsError::Eval(err) => err.into(),
        _ => panic!("Unexpected JsError variant"),
    }
}

pub async fn update_map(
    map_id: MapId, initial_position: &MapPosition, options: &MapOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_map(map_id, initial_position, options).await.map_err(js_to_eval)
}

pub async fn delete_map(map_id: MapId) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_map(map_id).await.map_err(js_to_eval)
}

pub async fn register_onclick_handler_map(
    map_id: MapId,
    event_handler: Option<EventHandler<LatLng>>
) -> Result<(), String> {
    let mut eval = document::eval(CALL_REGISTER_ONCLICK_HANDLER_MAP_JS);

    eval.send(map_id)
        .map_err(|e| e.to_string())?;

    if let Some(handler) = event_handler {
        spawn(async move {
            loop {
                if let Ok(Some(ret)) = eval.recv::<Option<LatLng>>().await {
                    handler.call(ret);
                }
            }
        });
    };
    Ok(())
}

pub async fn update_marker(
    map_id: MapId, 
    marker_id: MarkerId,
    coordinate: &LatLng,
    icon: &Option<MarkerIcon>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_marker(map_id, marker_id, coordinate, icon).await.map_err(js_to_eval)
}

pub async fn delete_marker(map_id: MapId, marker_id: MarkerId) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::delete_marker(map_id, marker_id).await.map_err(js_to_eval)
}

#[derive(Serialize)]
struct PolygonProps<'a> {
    pub map_id: usize,
    pub polygon_id: usize,
    pub coordinates: &'a Vec<LatLng>,
    pub options: &'a PathOptions,
}

pub async fn update_polygon(
    map_id: usize,
    polygon_id: usize,
    coordinates: &Vec<LatLng>,
    options: &PathOptions,
) -> Result<(), String> {
    let mut eval = document::eval(CALL_POLYGON_JS);

    eval.send(PolygonProps { map_id, polygon_id, coordinates, options })
        .map_err(|e| e.to_string())?;

    let ret = eval.recv::<Option<String>>().await
        .map_err(|e| e.to_string())?;

    if let Some(e) = ret {
        Err(e)
    }
    else {
        Ok(())
    }
}

pub async fn update_popup(
    marker_id: MarkerId,
    body_id: usize,
    options: &PopupOptions,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    js_api::update_popup(marker_id.into(), body_id as f64, options).await.map_err(js_to_eval)
}