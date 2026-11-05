use serde::{Deserialize, Serialize};

use crate::LatLng;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LatLngBounds {
    pub southwest: LatLng,
    pub northeast: LatLng,
}

impl LatLngBounds {
    pub fn new(southwest: LatLng, northeast: LatLng) -> Self {
        Self {
            southwest,
            northeast,
        }
    }
}
