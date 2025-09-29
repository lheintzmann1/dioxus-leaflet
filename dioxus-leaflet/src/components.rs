use dioxus::prelude::*;
use crate::types::*;
use crate::interop::{DL_JS, update};

/// Generates a unique map ID
pub fn generate_map_id() -> String {
    format!("dioxus_leaflet_map_{}", fastrand::u32(..))
}

#[derive(Props, Clone, PartialEq)]
pub struct MapProps {
    /// Initial position of the map
    #[props(default = MapPosition::default())]
    pub initial_position: MapPosition,
    
    /// Markers to display on the map
    #[props(default = vec![])]
    pub markers: ReadOnlySignal<Vec<MapMarker>>,
    
    /// Height of the map container
    #[props(default = "500px".to_string())]
    pub height: String,
    
    /// Width of the map container
    #[props(default = "100%".to_string())]
    pub width: String,
    
    /// Map configuration options
    #[props(default = MapOptions::default())]
    pub options: MapOptions,
    
    /// Custom CSS class for the map container
    #[props(default = "".to_string())]
    pub class: String,
    
    /// Custom CSS styles for the map container
    #[props(default = "".to_string())]
    pub style: String,
    
    /// Callback when marker is clicked
    pub on_marker_click: Option<EventHandler<MapMarker>>,
    
    /// Callback when map is clicked
    pub on_map_click: Option<EventHandler<MapPosition>>,
    
    /// Callback when map is moved
    pub on_map_move: Option<EventHandler<MapPosition>>,
}

/// Main map component using Leaflet
#[component]
pub fn Map(props: MapProps) -> Element {
    let map_id: Signal<String> = use_signal(generate_map_id);
    let mut load_error: Signal<Option<String>> = use_signal(|| None);
    
    let container_style = format!(
        "position: relative; width: {}; height: {}; {}",
        props.width, props.height, props.style
    );
    
    let container_class = if props.class.is_empty() {
        "dioxus-leaflet-container".to_string()
    } else {
        format!("dioxus-leaflet-container {}", props.class)
    };

    let css_path = if let Some(_) = props.options.leaflet_resources.css_integrity() {
        props.options.leaflet_resources.css_url()
    } else {
        props.options.leaflet_resources.css_url()
    };

    let js_path = if let Some(_) = props.options.leaflet_resources.js_integrity() {
        props.options.leaflet_resources.js_url()
    } else {
        props.options.leaflet_resources.js_url()
    };

    let onmounted = move |_| {
        let id = (&*map_id.read()).clone();
        let pos = props.initial_position.clone();
        let markers = props.markers.clone();
        let opts = props.options.clone();

        async move {
            if let Err(e) = update(&id, &pos, &*markers.read(), &opts).await {
                load_error.set(Some(e));
            }
        }
    };

    rsx! {
        // Leaflet CSS
        document::Style { href: css_path }
        
        // Leaflet JavaScript
        document::Script { src: js_path }
        
        // boot logic
        document::Script { src: DL_JS }

        if let Some(err) = &*load_error.read() {
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
                    id: "{map_id}",
                    class: "dioxus-leaflet-map",
                    style: "width: 100%; height: 100%; z-index: 1;",
                    onmounted: onmounted,
                }
            }
        }
    }
}
