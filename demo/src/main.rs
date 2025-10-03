use dioxus::prelude::*;
use dioxus_leaflet::{Color, Map, Marker, MapPosition, Popup, LatLng, PathOptions, Polygon};

mod jersey;

const CSS: Asset = asset!("/assets/demo.scss");

#[component]
fn App() -> Element {
    let mut markers = use_signal(|| vec![
        ("London", "Capital of the UK", LatLng::new(51.505, -0.09)),
        ("Paris", "Capital of France", LatLng::new(48.8566, 2.3522)),
        ("Berlin", "Capital of Germany", LatLng::new(52.52, 13.4)),
    ]);

    rsx! {
        document::Style { href: CSS }
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            for i in 0..markers.len() {
                Marker {
                    coordinate: markers.map(move |v| &v[i].2),
                    Popup {
                        b { "{&markers.get(i).unwrap().0}" }
                        br {}
                        "{&markers.get(i).unwrap().1}"
                    }
                }
            }
            Polygon {
                coordinates: Vec::from(&jersey::JERSEY_BORDER),
                options: PathOptions {
                    color: Color::new([1., 1., 0.]),
                    fill: true,
                    fill_color: Color::new([1., 1., 0.]),
                    ..Default::default()
                },
                Popup {
                    b { "Jersey" }
                    br {}
                    "Jersey"
                }
            } 
        }
        button {
            onclick: move |_| {
                if markers.len() > 1 {
                    markers.remove(markers.len() - 1);
                }
            },
            "Remove Marker"
        }
        button {
            onclick: move |_| {
                // if markers.len() > 0 {
                //     let mut new_marker = markers.get(markers.len() - 1).unwrap().clone();
                //     new_marker.coordinates += LatLng::new(0.5, 0.5);
                //     markers.push(new_marker);
                // }
            },
            "Add Marker"
        }
        button {
            onclick: move |_| {
                // for mut m in markers.iter_mut() {
                //     m.r#type = match m.r#type {
                //         MarkerType::Pin => MarkerType::Circle(CircleMarkerOptions::default()),
                //         MarkerType::Circle { .. } => MarkerType::Pin,
                //     };
                // }
            },
            "Switch Types"
        }
    }
}

fn main() {
    dioxus::launch(App);
}