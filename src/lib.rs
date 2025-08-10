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
//! - CDN-based Leaflet integration
//! 
//! ## Basic Usage
//! 
//! ```rust
//! use dioxus::prelude::*;
//! use dioxus_leaflet::{Map, MapPosition, MapMarker};
//! 
//! fn App() -> Element {
//!     let markers = vec![
//!         MapMarker {
//!             lat: 51.505,
//!             lng: -0.09,
//!             title: "London".to_string(),
//!             description: Some("Capital of England".to_string()),
//!             ..Default::default()
//!         }
//!     ];
//! 
//!     rsx! {
//!         Map {
//!             initial_position: MapPosition::default(),
//!             markers: markers,
//!             height: "400px",
//!             width: "100%"
//!         }
//!     }
//! }
//! ```

pub mod components;
pub mod types;
pub mod utils;

// Re-export main types and components
pub use components::*;
pub use types::*;
pub use utils::*;
