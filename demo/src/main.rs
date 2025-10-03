use dioxus::prelude::*;
use dioxus_leaflet::{Color, Map, Marker, MarkerProps, MapPosition, Popup, LatLng, PathOptions, Polygon};

mod jersey;

const CSS: Asset = asset!("/assets/demo.scss");

#[component]
fn App() -> Element {
    let mut markers = use_signal(|| vec![
        LatLng::new(51.505, -0.09),
        LatLng::new(48.8566, 2.3522),
        LatLng::new(52.52, 13.4),
    ]);

    let polygons = vec![
        Polygon {
            coordinates: jersey::JERSEY_BORDER.into(),
            title: "Jersey".into(),
            path_options: Some(PathOptions {
                color: Color::new([1., 1., 0.]),
                fill_color: Color::new([1., 1., 0.]),
                ..Default::default()
            }),
            ..Default::default()
        },
    ];

    rsx! {
        document::Style { href: CSS }
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            for i in 0..markers.len() {
                Marker {
                    coordinate: markers.map(move |v| &v[i]),
                    Popup {
                        b { "Title {i}" }
                        br {}
                        "Description {i}"
                    }
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