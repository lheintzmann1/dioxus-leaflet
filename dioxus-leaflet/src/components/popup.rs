use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use crate::{interop, PopupOptions};

#[derive(Debug, Clone, Copy)]
pub struct PopupContext(pub usize);

#[component]
pub fn Popup(
    #[props(default = PopupOptions::default())]
    options: PopupOptions,

    children: Element,
) -> Element {
    let marker: PopupContext = use_context();
    let popup_id = dioxus_core::current_scope_id().unwrap().0;
    let class = options.class_name.clone();

    use_effect(move || {
        let opts = options.clone();
        spawn(async move {
            if let Err(e) = interop::update_popup(marker.0, popup_id, &opts).await {
                error!("{e}");
            }
        });
    });

    rsx!(
        div {
            id: "dioxus-leaflet-popup-{popup_id}",
            class: "leaflet-popup-content {class.as_ref().map(|c| c.as_str()).unwrap_or(\"\")}",
            {children}
        }
    )
}

