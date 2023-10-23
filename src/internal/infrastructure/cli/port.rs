use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PortInformation {
    id: String,
    name: String,
    city: String,
    country: String,
    alias: Vec<String>,
    regions: Vec<String>,
    coordinates: [f64; 2],
    province: String,
    timezone: String,
    unlocs: Vec<String>,
    code: String,
}

impl PortInformation {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn city(&self) -> &str {
        &self.city
    }
    pub fn country(&self) -> &str {
        &self.country
    }
    pub fn alias(&self) -> &Vec<String> {
        &self.alias
    }
    pub fn regions(&self) -> &Vec<String> {
        &self.regions
    }
    pub fn coordinates(&self) -> [f64; 2] {
        self.coordinates
    }
    pub fn province(&self) -> &str {
        &self.province
    }
    pub fn timezone(&self) -> &str {
        &self.timezone
    }
    pub fn unlocs(&self) -> &Vec<String> {
        &self.unlocs
    }
    pub fn code(&self) -> &str {
        &self.code
    }
}
