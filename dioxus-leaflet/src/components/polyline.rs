use std::rc::Rc;

use dioxus::{
    core::{spawn_forever, use_drop},
    prelude::*,
};
use dioxus_logger::tracing::error;

use crate::{LatLng, PathOptions, interop, types::Id};

#[component]
pub fn Polyline(
    coordinates: ReadSignal<Vec<Vec<LatLng>>>,
    #[props(default = PathOptions::default())] options: ReadSignal<PathOptions>,
    children: Element,
) -> Element {
    let id: Rc<Id> = use_context();
    let id = use_context_provider(|| Rc::new(Id::polyline(&id, dioxus_core::current_scope_id().0)));

    let id2 = id.clone();
    use_effect(move || {
        let id = id2.clone();
        let coords = coordinates();
        let opts = options();
        spawn(async move {
            if let Err(e) = interop::update_polyline(&id, &coords, &opts).await {
                error!("{e}");
            }
        });
    });

    let id2 = id.clone();
    use_drop(move || {
        let id = id2.clone();
        spawn_forever(async move {
            if let Err(e) = interop::delete_polyline(&id).await {
                error!("{e}");
            }
        });
    });

    rsx!({ children })
}
