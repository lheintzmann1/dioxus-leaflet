use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use std::rc::Rc;
use crate::{interop, PopupOptions, types::Id};

#[component]
pub fn Popup(
    #[props(default = PopupOptions::default())]
    options: PopupOptions,

    children: Element,
) -> Element {
    let id: Rc<Id> = use_context();
    let id = Id::popup(&id, dioxus_core::current_scope_id().0);
    let class = options.class_name.clone();

    let id2 = id.clone();
    use_effect(move || {
        let id = id2.clone();
        let opts = options.clone();
        spawn(async move {
            if let Err(e) = interop::update_popup(&id, &opts).await {
                error!("{e}");
            }
        });
    });

    rsx!(
        div {
            id: "dioxus-leaflet-{id}",
            class: "dioxus-leaflet-popup-content {class.as_ref().map(|c| c.as_str()).unwrap_or(\"\")}",
            {children}
        }
    )
}

