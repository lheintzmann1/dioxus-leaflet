use dioxus::prelude::*;
use dioxus_logger::tracing::error;

use crate::{components::{map::MapContext, popup::PopupContext}, interop, LatLng, PathOptions};

#[component]
pub fn Polygon(
    coordinates: ReadOnlySignal<Vec<LatLng>>,

    #[props(default = PathOptions::default())]
    options: ReadOnlySignal<PathOptions>,

    children: Option<Element>,
) -> Element {
    let map_id: MapContext = use_context();
    let my_id = dioxus_core::current_scope_id().unwrap().0;
    use_context_provider(|| PopupContext(my_id));

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