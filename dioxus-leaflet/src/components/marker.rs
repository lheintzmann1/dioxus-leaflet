use dioxus::prelude::*;
use std::collections::HashMap;
use dioxus_logger::tracing::error;

use crate::{components::map::MapContext, LatLng, MarkerIcon, interop};

#[derive(Debug, Clone, Copy)]
pub struct MarkerContext(pub usize);

#[component]
pub fn Marker(
    coordinate: ReadOnlySignal<LatLng>,

    #[props(default = None)]
    icon: ReadOnlySignal<Option<MarkerIcon>>,

    #[props(default = None)]
    custom_data: ReadOnlySignal<Option<HashMap<String, String>>>,
) -> Element {
    let map: MapContext = use_context();
    let context: MarkerContext = use_context_provider(|| MarkerContext(dioxus_core::current_scope_id().unwrap().0));

    use_effect(move || {
        let coord = coordinate();
        let icon = icon();
        spawn(async move {
            if let Err(e) = interop::update_marker(map.0, context.0, &coord, &icon).await {
                error!("Error rendering marker: {e}");
            }
        });
    });

    rsx!()
}