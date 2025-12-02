use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use serde::Serialize;

use crate::{components::{map::MapId, popup::PopupContext}, interop, LatLng, PathOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct PolygonId(pub usize);
impl PopupContext for PolygonId {
    fn popup_id(&self) -> usize {
        self.0
    }
}
impl Into<f64> for PolygonId {
    fn into(self) -> f64 {
        self.0 as f64
    }
}
impl From<f64> for PolygonId {
    fn from(value: f64) -> Self {
        PolygonId(value as usize)
    }
}

#[component]
pub fn Polygon(
    coordinates: ReadSignal<Vec<LatLng>>,

    #[props(default = PathOptions::default())]
    options: ReadSignal<PathOptions>,

    children: Option<Element>,
) -> Element {
    let map_id: MapId = use_context();
    let my_id = use_context_provider(|| PolygonId(dioxus_core::current_scope_id().0));

    use_effect(move || {
        let coords = coordinates();
        let opts = options();
        spawn(async move {
            if let Err(e) = interop::update_polygon(map_id.0, my_id, &coords, &opts).await {
                error!("{e}");
            }
        });
    });

    rsx!({children})
}