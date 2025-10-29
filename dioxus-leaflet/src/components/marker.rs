use dioxus::{core::use_drop, prelude::*};
use std::collections::HashMap;
use dioxus_logger::tracing::error;

use crate::{
    components::map::MapContext, 
    components::popup::PopupContext,
    LatLng, 
    MarkerIcon, 
    interop,
};

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
    let map: MapContext = use_context();
    let context = use_context_provider(|| PopupContext(dioxus_core::current_scope_id().0));

    use_effect(move || {
        let coord = coordinate();
        let icon = icon();
        spawn(async move {
            if let Err(e) = interop::update_marker(map.0, context.0, &coord, &icon).await {
                error!("Error rendering marker: {e}");
            }
        });
    });

    use_drop(move || {
        let marker_id = interop::MarkerId{
            map_id: map.0,
            marker_id: context.0
        };
        if let Err(e) = interop::delete_marker(marker_id) {
            error!("Error deleting marker: {e}");
        }
    });

    rsx!({children})
}