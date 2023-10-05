pub struct Coordinates {
    latitude: f64,
    longitude: f64
}

impl Coordinates {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude: latitude, 
            longitude: longitude
        }
    }

}