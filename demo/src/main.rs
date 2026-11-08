use dioxus::prelude::*;
use dioxus_leaflet::{
    Color, ImageOverlay, ImageOverlayOptions, LatLng, LatLngBounds, Map, MapPosition, Marker,
    PathOptions, Polygon, Polyline, Popup,
};

mod jersey;

const CSS: Asset = asset!("/assets/demo.scss");

/// A simple inline SVG used as an image overlay example.
const OVERLAY_IMAGE: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='200' height='120'><rect width='200' height='120' fill='orange'/><text x='100' y='65' font-size='24' text-anchor='middle' fill='white'>Overlay</text></svg>";

#[component]
fn App() -> Element {
    let mut markers = use_signal(|| {
        vec![
            ("London", "Capital of the UK", LatLng::new(51.505, -0.09)),
            ("Paris", "Capital of France", LatLng::new(48.8566, 2.3522)),
            ("Berlin", "Capital of Germany", LatLng::new(52.52, 13.4)),
        ]
    });

    let route = use_signal(|| {
        vec![vec![
            LatLng::new(51.505, -0.09),
            LatLng::new(48.8566, 2.3522),
            LatLng::new(52.52, 13.4),
        ]]
    });

    let mut overlay_opacity = use_signal(|| 0.7);

    rsx! {
        document::Style { href: CSS }
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            on_click: move |pos: LatLng| {
                info!("Map clicked at: {:?}", pos);
                if let Some(marker) = markers.write().last_mut() {
                    marker.2 = pos;
                }
            },
            on_move: move |pos: MapPosition| {
                info!("Map moved to: {:?}", pos);
            },
            for marker in markers() {
                Marker {
                    coordinate: marker.2,
                    Popup {
                        b { "{marker.0}"}
                        br {}
                        "{marker.1}"
                    }
                }
            }
            ImageOverlay {
                image_url: OVERLAY_IMAGE,
                bounds: LatLngBounds::new(LatLng::new(50.0, -2.0), LatLng::new(52.0, 3.0)),
                options: ImageOverlayOptions::default().with_opacity(overlay_opacity()),
            }
            Polyline {
                coordinates: route,
                options: PathOptions {
                    color: Color::new([0., 1., 1.]),
                    weight: 5,
                    ..Default::default()
                },
                Popup { "Route connecting capitals" }
            }
            Polygon {
                coordinates: vec![vec![Vec::from(&jersey::JERSEY_BORDER)]],
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
        input {
            r#type: "range",
            min: "0",
            max: "1",
            step: "0.1",
            value: "{overlay_opacity}",
            oninput: move |e| {
                if let Ok(v) = e.value().parse() {
                    overlay_opacity.set(v);
                }
            },
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
                if let Some(last) = markers().last() {
                    let new_lat_lng = last.2 + LatLng::new(0.5, 0.5);
                    markers.write().push((
                        "New City",
                        "A new location",
                        new_lat_lng
                    ));
                }
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
