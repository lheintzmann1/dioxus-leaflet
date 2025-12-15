use dioxus::{core::use_drop, prelude::*};
use dioxus_logger::tracing::error;
use std::{collections::HashMap, rc::Rc};

use crate::{LatLng, MarkerIcon, interop, types::Id};

#[component]
pub fn Marker(
    coordinate: ReadSignal<LatLng>,

    #[props(default = None)] icon: ReadSignal<Option<MarkerIcon>>,

    #[props(default = None)] custom_data: ReadSignal<Option<HashMap<String, String>>>,

    on_click: Option<EventHandler>,

    children: Element,
) -> Element {
    let map: Rc<Id> = use_context();
    let id = use_context_provider(|| Rc::new(Id::marker(&map, dioxus_core::current_scope_id().0)));

    let id2 = id.clone();
    use_effect(move || {
        let id = id2.clone();
        let coord = coordinate();
        let icon = icon();
        spawn(async move {
            if let Err(e) = interop::update_marker(&id, &coord, &icon).await {
                error!("Error rendering marker: {e}");
            }
        });
    });

    let id2 = id.clone();
    use_drop(move || {
        let id = id2.clone();
        spawn(async move {
            if let Err(e) = interop::delete_marker(&id).await {
                error!("Error deleting marker: {e}");
            }
        });
    });

    rsx!({ children })
}
