use std::rc::Rc;

use crate::{
    interop,
    types::{Id, ImageOverlayOptions, LatLngBounds},
};
use dioxus::{core::spawn_forever, prelude::*};

#[component]
pub fn ImageOverlay(
    image_url: ReadSignal<String>,
    bounds: ReadSignal<LatLngBounds>,
    #[props(default = ImageOverlayOptions::default())] options: ImageOverlayOptions,
    children: Element,
) -> Element {
    let map: Rc<Id> = use_context();
    let id = use_context_provider(|| {
        Rc::new(Id::image_overlay(&map, dioxus_core::current_scope_id().0))
    });

    let id2 = id.clone();
    use_resource(move || {
        let id = id2.clone();
        let image_url = image_url.cloned();
        let bounds = bounds.cloned();
        let options = options.clone();
        async move {
            if let Err(e) = interop::update_imageoverlay(&id, &image_url, &bounds, &options).await {
                error!("Error rendering imageoverlay: {e}");
            }
        }
    });

    let id2 = id.clone();
    use_drop(move || {
        let id = id2.clone();
        spawn_forever(async move {
            if let Err(e) = interop::delete_imageoverlay(&id).await {
                error!("Error deleting image overlay: {e}");
            }
        });
    });

    rsx!(
        {children}

    )
}
