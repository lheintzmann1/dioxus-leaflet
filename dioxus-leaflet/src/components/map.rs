use dioxus::prelude::*;
use crate::{interop, MapOptions, MapPosition};

#[derive(Debug, Clone, Copy)]
pub struct MapContext(pub usize);

/// Main map component using Leaflet
#[component]
pub fn Map(
    /// Initial position of the map
    #[props(default = MapPosition::default())]
    initial_position: MapPosition,
    
    /// Height of the map container
    #[props(default = "500px".to_string())]
    height: String,
    
    /// Width of the map container
    #[props(default = "100%".to_string())]
    width: String,
    
    /// Map configuration options
    #[props(default = MapOptions::default())]
    options: MapOptions,
    
    /// Custom CSS class for the map container
    #[props(default = "".to_string())]
    class: String,
    
    /// Custom CSS styles for the map container
    #[props(default = "".to_string())]
    style: String,
    
    /// Callback when map is clicked
    on_map_click: Option<EventHandler<MapPosition>>,
    
    /// Callback when map is moved
    on_map_move: Option<EventHandler<MapPosition>>,

    children: Element,
) -> Element {
    let context = use_context_provider(|| MapContext(dioxus_core::current_scope_id().unwrap().0));
    let mut load_error: Signal<Option<String>> = use_signal(|| None);
    
    let container_style = format!(
        "position: relative; width: {}; height: {}; {}",
        width, height, style
    );
    
    let container_class = if class.is_empty() {
        "dioxus-leaflet-container".to_string()
    } else {
        format!("dioxus-leaflet-container {}", class)
    };

    let css_path = if let Some(_) = options.leaflet_resources.css_integrity() {
        options.leaflet_resources.css_url()
    } else {
        options.leaflet_resources.css_url()
    };

    let js_path = if let Some(_) = options.leaflet_resources.js_integrity() {
        options.leaflet_resources.js_url()
    } else {
        options.leaflet_resources.js_url()
    };

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
        document::Style { href: css_path }
        
        // Leaflet JavaScript
        document::Script { src: js_path }
        
        // boot logic
        document::Script { src: interop::DL_JS }

        if let Some(err) = load_error() {
            p {
                "{err}"
            }
        }
        else {
            div {
                class: "{container_class}",
                style: "{container_style}",

                // Map container
                div {
                    id: "dioxus-leaflet-{context.0}",
                    class: "dioxus-leaflet-map",
                    style: "width: 100%; height: 100%; z-index: 1;",

                    {children}
                }
            }
        }
    }
}
