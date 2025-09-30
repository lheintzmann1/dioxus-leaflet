use dioxus::prelude::*;
use dioxus_leaflet::{Color, CircleMarkerOptions, Map, MapMarker, MapPosition, MarkerType, PathOptions, Polygon};

mod jersey;

#[component]
fn App() -> Element {
    let mut markers = use_signal(|| vec![
        MapMarker::new(51.505, -0.09, "London")
            .with_description("Capital of England"),
        MapMarker::new(48.8566, 2.3522, "Paris")
            .with_description("Capital of France")
            .with_circle_options(CircleMarkerOptions::default()),
        MapMarker::new(52.52, 13.4, "Berlin")
            .with_description("Capital of Germany")
            .with_circle_options(CircleMarkerOptions {
                radius: 15,
                path_options: PathOptions {
                    color: Color::new([0.8, 0.1, 0.8]),
                    weight: 5,
                    fill: true,
                    fill_color: Color::new([0., 1., 0.]),
                    ..Default::default()
                },
            }),
    ]);

    rsx! {
        Map {
            initial_position: MapPosition::new(51.505, -0.09, 5.0),
            markers: markers,
            polygons: vec![
                Polygon {
                    points: jersey::JERSEY_BORDER.into(),
                    title: "Jersey".into(),
                    path_options: Some(PathOptions {
                        color: Color::new([1., 1., 0.]),
                        fill_color: Color::new([1., 1., 0.]),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            height: "500px",
            width: "100%"
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
                if markers.len() > 0 {
                    let mut new_marker = markers.get(markers.len() - 1).unwrap().clone();
                    new_marker.lat += 0.5;
                    new_marker.lng += 0.5;
                    markers.push(new_marker);
                }
            },
            "Add Marker"
        }
        button {
            onclick: move |_| {
                for mut m in markers.iter_mut() {
                    m.r#type = match m.r#type {
                        MarkerType::Pin => MarkerType::Circle(CircleMarkerOptions::default()),
                        MarkerType::Circle { .. } => MarkerType::Pin,
                    };
                }
            },
            "Switch Types"
        }
    }
}

fn main() {
    dioxus::launch(App);
}