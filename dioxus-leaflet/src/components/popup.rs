use dioxus::prelude::*;
use dioxus_logger::tracing::error;
use crate::{interop, PopupOptions, components::MarkerId};

pub trait PopupContext {
    fn popup_id(&self) -> usize;
}
impl PopupContext for usize {
    fn popup_id(&self) -> usize {
        *self
    }
}

#[component]
pub fn Popup(
    #[props(default = PopupOptions::default())]
    options: PopupOptions,

    children: Element,
) -> Element {
    let marker: MarkerId = use_context();
    let popup_id = dioxus_core::current_scope_id().0.into();
    let class = options.class_name.clone();

    use_effect(move || {
        let opts = options.clone();
        spawn(async move {
            if let Err(e) = interop::update_popup(marker, popup_id, &opts).await {
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

