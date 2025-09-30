use dioxus::prelude::*;
use dioxus_leaflet::{Map, MapPosition, MapMarker};

#[component]
fn App() -> Element {
    let markers = vec![
        MapMarker::new(51.505, -0.09, "London")
            .with_description("Capital of England"),
        MapMarker::new(48.8566, 2.3522, "Paris")
            .with_description("Capital of France"),
    ];

    rsx! {
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            markers: markers,
            height: "500px",
            width: "100%"
        }
    }
}

fn main() {
    dioxus::launch(App);
}