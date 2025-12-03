use dioxus::{core::use_drop, prelude::*};
use std::rc::Rc;
use crate::{LatLng, MapOptions, MapPosition, interop, types::Id};

const MAP_CSS: Asset = asset!("/assets/dioxus_leaflet.scss");

/// Main map component using Leaflet
#[component]
pub fn Map(
    /// Initial position of the map
    #[props(default = MapPosition::default())]
    initial_position: MapPosition,
    
    /// Height of the map container
    #[props(into)]
    height: Option<String>,
    
    /// Width of the map container
    #[props(into)]
    width: Option<String>,
    
    /// Map configuration options
    options: Option<MapOptions>,
    
    /// Custom CSS class for the map container
    #[props(into)]
    class: Option<String>,
    
    /// Custom CSS styles for the map container
    #[props(into)]
    style: Option<String>,
    
    /// Callback when map is clicked
    on_click: Option<EventHandler<LatLng>>,
    
    /// Callback when map is moved
    on_move: Option<EventHandler<MapPosition>>,

    children: Option<Element>,
) -> Element {
    let id = use_context_provider(|| Rc::new(Id::map(dioxus_core::current_scope_id().0)));
    let mut load_error: Signal<Option<String>> = use_signal(|| None);
    let options = options.unwrap_or(MapOptions::default());
    let leaflet_css = options.leaflet_resources.css_url();
    let leaflet_js = options.leaflet_resources.js_url();

    let id2 = id.clone();
    use_effect(move || {
        let id = id2.clone();
        let pos = initial_position.clone();
        let opts = options.clone();
        spawn(async move {
            if let Err(e) = interop::update_map(&id, &pos, &opts).await {
                load_error.set(Some(e.to_string()));
            }
            if let Err(e) = interop::register_onclick_handler_map(&id, on_click).await {
                load_error.set(Some(e.to_string()));
            }
        });
    });

    let id2 = id.clone();
    use_drop(move || { 
        let id = id2.clone();
        spawn(async move {
            if let Err(e) = interop::delete_map(&id).await {
                load_error.set(Some(e.to_string()));
            }
        });
    });

    rsx! {
        // Leaflet CSS
        document::Style { href: leaflet_css }

        document::Style { href: MAP_CSS }
        
        // Leaflet JavaScript
        document::Script { src: leaflet_js }
        
        // boot logic
        document::Script { src: interop::DL_JS }

        if let Some(err) = load_error() {
            p {
                "{err}"
            }
        }
        else {
            // Map container
            div {
                class: "dioxus-leaflet-container {class.as_ref().map(|c| c.as_str()).unwrap_or(\"\")}",

                // Element taken over by leaflet
                div {
                    id: "dioxus-leaflet-{id}",
                    class: "dioxus-leaflet-map",
                    {children}
                }
            }
        }
    }
}
