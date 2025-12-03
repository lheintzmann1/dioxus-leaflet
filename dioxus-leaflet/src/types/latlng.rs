use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
    pub alt: Option<f64>,
}

impl LatLng {
    pub const fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng, alt: None }
    }
}

impl std::ops::Add for LatLng {
    type Output = LatLng;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            lat: self.lat + rhs.lat,
            lng: self.lng + rhs.lng,
            alt: match (self.alt, rhs.alt) {
                (Some(a), Some(b)) => Some(a + b),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            },
        }
    }
}

impl std::ops::AddAssign for LatLng {
    fn add_assign(&mut self, rhs: Self) {
        self.lat += rhs.lat;
        self.lng += rhs.lng;
        self.alt = match (self.alt, rhs.alt) {
            (Some(a), Some(b)) => Some(a + b),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
    }
}