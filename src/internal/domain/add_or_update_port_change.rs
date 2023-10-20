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
}