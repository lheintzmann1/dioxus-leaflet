use dioxus::prelude::*;
use dioxus_leaflet::{Map, MapPosition, MapMarker};

#[component]
fn App() -> Element {
    let mut markers = use_signal(|| vec![
        MapMarker::new(51.505, -0.09, "London")
            .with_description("Capital of England"),
        MapMarker::new(48.8566, 2.3522, "Paris")
            .with_description("Capital of France"),
    ]);

    rsx! {
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            markers: markers,
            height: "500px",
            width: "100%"
        }
        button {
            onclick: move |_| {
                if markers.len() > 1 {
                    markers.remove(markers.len() - 1);
                }
            },
            "Remove One"
        }
        button {
            onclick: move |_| {
                if markers.len() > 0 {
                    let mut new_marker = markers.get(markers.len() - 1).unwrap().clone();
                    new_marker.lat += 0.5;
                    new_marker.lng += 0.5;
                    markers.push(new_marker);
                }
            },
            "Add One"
        }
    }
}

fn main() {
    dioxus::launch(App);
}