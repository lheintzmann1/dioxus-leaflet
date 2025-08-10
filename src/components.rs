use dioxus::prelude::*;
use crate::types::*;
use crate::utils::*;

#[derive(Props, Clone, PartialEq)]
pub struct MapProps {
    /// Initial position of the map
    #[props(default = MapPosition::default())]
    pub initial_position: MapPosition,
    
    /// Markers to display on the map
    #[props(default = vec![])]
    pub markers: Vec<MapMarker>,
    
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
    let map_id: Signal<String> = use_signal(|| generate_map_id());
    let map_id_clone: String = map_id.read().clone();
    
    let init_script: String = generate_map_script(
        &map_id_clone,
        &props.initial_position,
        &props.markers,
        &props.options,
    );
    
    let container_style = format!(
        "position: relative; width: {}; height: {}; {}",
        props.width, props.height, props.style
    );
    
    let container_class = if props.class.is_empty() {
        "dioxus-leaflet-container".to_string()
    } else {
        format!("dioxus-leaflet-container {}", props.class)
    };
    
    rsx! {
        div {
            class: "{container_class}",
            style: "{container_style}",
            
            // Leaflet CSS
            link {
                rel: "stylesheet",
                href: "https://unpkg.com/leaflet@1.9.4/dist/leaflet.css",
                integrity: "sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=",
                crossorigin: ""
            }
            
            // Map container
            div {
                id: "{map_id_clone}",
                class: "dioxus-leaflet-map",
                style: "width: 100%; height: 100%; z-index: 1;",
            }
            
            // Leaflet JavaScript
            script {
                src: "https://unpkg.com/leaflet@1.9.4/dist/leaflet.js",
                integrity: "sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=",
                crossorigin: ""
            }
            
            // Initialize map script
            script {
                dangerous_inner_html: "{init_script}"
            }
        }
    }
}
