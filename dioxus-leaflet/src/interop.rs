use dioxus::prelude::*;
use serde::Serialize;

use crate::{MapMarker, MapOptions, MapPosition};

pub const DL_JS: Asset = asset!("/assets/initialize_dioxus_leaflet_map.js");
const CALL_INIT_JS: &str = r#"
while (!window.L || !window.DioxusLeaflet) {
    await new Promise(cb => setTimeout(cb, 300));
}
window.DioxusLeaflet.initializeAsync(() => dioxus.recv(), (x) => dioxus.send(x));
"#;

#[derive(Serialize)]
struct InitProps<'a> {
    map_id: &'a str,
    initial_position: &'a MapPosition,
    markers: &'a Vec<MapMarker>,
    options: &'a MapOptions,
}

/// Generates a unique map ID
pub fn generate_map_id() -> String {
    format!("dioxus_leaflet_map_{}", fastrand::u32(..))
}

pub async fn initialize(
    map_id: &str, 
    initial_position: &MapPosition, 
    markers: &Vec<MapMarker>, 
    options: &MapOptions,
) -> Result<(), String> {
    let mut eval = document::eval(CALL_INIT_JS);

    eval.send(InitProps { map_id, initial_position, markers, options })
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