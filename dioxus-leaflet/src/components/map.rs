use dioxus::prelude::*;
use crate::{interop, MapOptions, MapPosition};

const MAP_CSS: Asset = asset!("/assets/dioxus_leaflet.scss");

#[derive(Debug, Clone, Copy)]
pub struct MapContext(pub usize);

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
    on_map_click: Option<EventHandler<MapPosition>>,
    
    /// Callback when map is moved
    on_map_move: Option<EventHandler<MapPosition>>,

    children: Option<Element>,
) -> Element {
    let context = use_context_provider(|| MapContext(dioxus_core::current_scope_id().unwrap().0));
    let mut load_error: Signal<Option<String>> = use_signal(|| None);
    let options = options.unwrap_or(MapOptions::default());
    let leaflet_css = options.leaflet_resources.css_url();
    let leaflet_js = options.leaflet_resources.js_url();

    use_effect(move || {
        let id = context.0;
        let pos = initial_position.clone();
        let opts = options.clone();
        spawn(async move {
            if let Err(e) = interop::update_map(id, &pos, &opts).await {
                load_error.set(Some(e));
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
                    id: "dioxus-leaflet-{context.0}",
                    class: "dioxus-leaflet-map",
                    {children}
                }
            }
        }
    }
}
