use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub const fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

impl std::ops::Add for LatLng {
    type Output = LatLng;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.lat + rhs.lat, self.lng + rhs.lng)
    }
}

impl std::ops::AddAssign for LatLng {
    fn add_assign(&mut self, rhs: Self) {
        self.lat += rhs.lat;
        self.lng += rhs.lng;
    }
}