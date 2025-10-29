use dioxus::prelude::*;
use serde::Serialize;

use crate::{LatLng, MapOptions, MapPosition, MarkerIcon, PathOptions, PopupOptions};

pub const DL_JS: Asset = asset!("/assets/dioxus_leaflet.js");

const CALL_MAP_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.updateMapAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

const DELETE_MAP_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.deleteMap(() => dioxus.recv());
"#;

const CALL_REGISTER_ONCLICK_HANDLER_MAP_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.registerOnClickHandlerMapAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

const CALL_MARKER_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.updateMarkerAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

const DELETE_MARKER_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.deleteMarker(() => dioxus.recv());
"#;

const CALL_POPUP_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.updatePopupAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

const CALL_POLYGON_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.updatePolygonAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

#[derive(Serialize)]
struct MapProps<'a> {
    map_id: usize,
    initial_position: &'a MapPosition,
    options: &'a MapOptions,
}

pub async fn update_map(
    map_id: usize, 
    initial_position: &MapPosition, 
    options: &MapOptions,
) -> Result<(), String> {
    let mut eval = document::eval(CALL_MAP_JS);

    eval.send(MapProps { map_id, initial_position, options })
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

pub fn delete_map(map_id: usize) -> Result<(), String> {
    let eval = document::eval(DELETE_MAP_JS);
    eval.send(MapId { map_id})
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct MapId {
    map_id: usize
}

pub async fn register_onclick_handler_map(
    map_id: usize,
    event_handler: Option<EventHandler<LatLng>>
) -> Result<(), String> {
    let mut eval = document::eval(CALL_REGISTER_ONCLICK_HANDLER_MAP_JS);

    eval.send(MapId { map_id })
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

#[derive(Serialize)]
struct MarkerProps<'a> {
    map_id: usize,
    marker_id: usize,
    coordinate: &'a LatLng,
    icon: &'a Option<MarkerIcon>,
}

pub async fn update_marker(
    map_id: usize, 
    marker_id: usize,
    coordinate: &LatLng,
    icon: &Option<MarkerIcon>,
) -> Result<(), String> {
    let mut eval = document::eval(CALL_MARKER_JS);

    eval.send(MarkerProps { map_id, marker_id, coordinate, icon })
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

#[derive(Serialize)]
pub struct MarkerId {
    pub map_id: usize,
    pub marker_id: usize
}

pub fn delete_marker(marker_id: MarkerId) -> Result<(), String> {
    let eval = document::eval(DELETE_MARKER_JS);
    eval.send(marker_id)
        .map_err(|e| e.to_string()) 
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

#[derive(Serialize)]
struct PopupProps<'a> {
    pub marker_id: usize,
    pub body_id: usize,
    pub options: &'a PopupOptions,
}

pub async fn update_popup(
    marker_id: usize,
    body_id: usize,
    options: &PopupOptions,
) -> Result<(), String> {
    let mut eval = document::eval(CALL_POPUP_JS);

    eval.send(PopupProps { marker_id, body_id, options })
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