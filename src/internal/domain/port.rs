use super::port_id::PortId;
use super::coordinates::Coordinates;

pub struct Port {
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

impl Port {
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
        Self {
            id: id,
            name: name,
            city: city,
            country: country,
            alias: alias,
            regions: regions,
            coordinates: coordinates,
            province: province,
            timezone: timezone,
            unlocs: unlocs,
            code: code,
        }
    }
}
