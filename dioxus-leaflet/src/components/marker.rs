use dioxus::{core::use_drop, prelude::*};
use std::collections::HashMap;
use dioxus_logger::tracing::error;
use serde::Serialize;

use crate::{
    components::map::MapId, 
    components::popup::PopupContext,
    LatLng, 
    MarkerIcon, 
    interop,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct MarkerId(pub usize);
impl PopupContext for MarkerId {
    fn popup_id(&self) -> usize {
        self.0
    }
}
impl Into<f64> for MarkerId {
    fn into(self) -> f64 {
        self.0 as f64
    }
}
impl From<f64> for MarkerId {
    fn from(value: f64) -> Self {
        MarkerId(value as usize)
    }
}

#[component]
pub fn Marker(
    coordinate: ReadSignal<LatLng>,

    #[props(default = None)]
    icon: ReadSignal<Option<MarkerIcon>>,

    #[props(default = None)]
    custom_data: ReadSignal<Option<HashMap<String, String>>>,

    on_click: Option<EventHandler>,

    children: Option<Element>,
) -> Element {
    let map: MapId = use_context();
    let id = use_context_provider(|| MarkerId(dioxus_core::current_scope_id().0));

    use_effect(move || {
        let coord = coordinate();
        let icon = icon();
        spawn(async move {
            if let Err(e) = interop::update_marker(map, id, &coord, &icon).await {
                error!("Error rendering marker: {e}");
            }
        });
    });

    use_drop(move || {
        spawn(async move {
            if let Err(e) = interop::delete_marker(map, id).await {
                error!("Error deleting marker: {e}");
            }
        });
    });

    rsx!({children})
}