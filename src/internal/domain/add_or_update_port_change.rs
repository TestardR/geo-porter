use crate::internal::domain::coordinates::Coordinates;
use crate::internal::domain::port_id::PortId;

pub struct AddOrUpdatePortChange {
    id: PortId,
    name: String,
    city: String,
    country: String,
    alias: Vec<String>,
    regions: Vec<String>,
    coordinates: Coordinates,
    province: String,
    timezone: String,
    unlocs: Vec<String>,
    code: String,
}

impl AddOrUpdatePortChange {
    pub fn new(
        id: PortId,
        name: String,
        city: String,
        country: String,
        alias: Vec<String>,
        regions: Vec<String>,
        coordinates: Coordinates,
        province: String,
        timezone: String,
        unlocs: Vec<String>,
        code: String,
    ) -> Self {
        Self { id, name, city, country, alias, regions, coordinates, province, timezone, unlocs, code }
    }

    pub fn id(&self) -> &PortId {
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

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
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