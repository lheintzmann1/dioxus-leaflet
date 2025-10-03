//! # Dioxus Leaflet
//! 
//! A general-purpose Leaflet map component for Dioxus applications.
//! 
//! ## Features
//! 
//! - Easy-to-use map component with customizable markers
//! - Support for popups and custom styling
//! - Extensible marker system
//! - TypeScript-like props system
//! - Flexible Leaflet integration: CDN (with version selection) or local files
//! - Configurable Leaflet resources with integrity checking
//! 
//! ## Basic Usage
//! 
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_leaflet::{Map, MapPosition, MapMarker};
//! 
//! fn App() -> Element {
//!     let markers = vec![
//!         MapMarker::new(51.505, -0.09, "London")
//!             .with_description("Capital of England")
//!     ];
//! 
//!     rsx! {
//!         Map {
//!             initial_position: MapPosition::new(51.505, -0.09, 13.0),
//!             markers: markers,
//!             height: "400px",
//!             width: "100%"
//!         }
//!     }
//! }
//! ```
//! 
//! ## Advanced Configuration
//! 
//! ### Using a specific Leaflet version from CDN
//! 
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_leaflet::{Map, MapPosition, MapOptions, LeafletResources};
//! 
//! fn App() -> Element {
//!     let options = MapOptions::default()
//!         .with_leaflet_resources(LeafletResources::cdn("1.9.3"));
//! 
//!     rsx! {
//!         Map {
//!             initial_position: MapPosition::default(),
//!             options: options,
//!             height: "400px",
//!             width: "100%"
//!         }
//!     }
//! }
//! ```
//! 
//! ### Using local Leaflet files
//! 
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_leaflet::{Map, MapPosition, MapOptions, LeafletResources};
//! 
//! fn App() -> Element {
//!     let options = MapOptions::default()
//!         .with_leaflet_resources(LeafletResources::local(
//!             "/static/css/leaflet.css",
//!             "/static/js/leaflet.js"
//!         ));
//! 
//!     rsx! {
//!         Map {
//!             initial_position: MapPosition::default(),
//!             options: options,
//!             height: "400px",
//!             width: "100%"
//!         }
//!     }
//! }
//! ```

mod components;
mod types;
mod interop;

// Re-export main types and components
pub use components::{
    Map, 
    Marker, 
    Polygon, 
    Popup,
};
pub use types::{
    Color, 
    LatLng, 
    LeafletResources, 
    MapOptions, 
    MapPosition, 
    MarkerIcon, 
    PathOptions, 
    PopupOptions, 
    TileLayer,
};
