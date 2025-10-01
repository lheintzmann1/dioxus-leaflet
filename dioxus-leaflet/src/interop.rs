use dioxus::prelude::*;
use serde::Serialize;

use crate::{MapMarker, MapOptions, MapPosition, Polygon};

pub const DL_JS: Asset = asset!("/assets/dioxus_leaflet.js");
const CALL_INIT_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 100));
}
await window.DioxusLeaflet.updateAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

#[derive(Serialize)]
struct UpdateProps<'a> {
    map_id: &'a str,
    initial_position: &'a MapPosition,
    markers: &'a Vec<MapMarker>,
    polygons: &'a Vec<Polygon>,
    options: &'a MapOptions,
}

pub async fn update(
    map_id: &str, 
    initial_position: &MapPosition, 
    markers: &Vec<MapMarker>,
    polygons: &Vec<Polygon>,
    options: &MapOptions,
) -> Result<(), String> {
    let mut eval = document::eval(CALL_INIT_JS);

    eval.send(UpdateProps { map_id, initial_position, markers, polygons, options })
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