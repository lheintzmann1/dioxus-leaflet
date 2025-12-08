use dioxus::{core::use_drop, prelude::*};
use dioxus_logger::tracing::error;
use std::rc::Rc;

use crate::{LatLng, PathOptions, interop, types::Id};

#[component]
pub fn Polygon(
    coordinates: ReadSignal<Vec<Vec<Vec<LatLng>>>>,

    #[props(default = PathOptions::default())] options: ReadSignal<PathOptions>,

    children: Option<Element>,
) -> Element {
    let id: Rc<Id> = use_context();
    let id = use_context_provider(|| Rc::new(Id::polygon(&id, dioxus_core::current_scope_id().0)));

    let id2 = id.clone();
    use_effect(move || {
        let id = id2.clone();
        let coords = coordinates();
        let opts = options();
        spawn(async move {
            if let Err(e) = interop::update_polygon(&id, &coords, &opts).await {
                error!("{e}");
            }
        });
    });

    let id2 = id.clone();
    use_drop(move || {
        let id = id2.clone();
        spawn(async move {
            if let Err(e) = interop::delete_polygon(&id).await {
                error!("{e}");
            }
        });
    });

    rsx!({ children })
}
