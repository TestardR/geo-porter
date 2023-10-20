use crate::internal::domain::add_or_update_port_change::AddOrUpdatePortChange;
use crate::internal::domain::errors::DomainViolation;
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
            id,
            name,
            city,
            country,
            alias,
            regions,
            coordinates,
            province,
            timezone,
            unlocs,
            code,
        }
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

pub fn add_or_update_port(
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
) -> Result<AddOrUpdatePortChange, DomainViolation> {
    let mut violations = vec![];

    if name == "" {
        violations.push(String::from("name is required"));
    }

    if city == "" {
        violations.push(String::from("city is required"));
    }

    if violations.len() > 0 {
        Err(DomainViolation::new(violations.join(", ")))
    }

    Ok(AddOrUpdatePortChange::new(
        id,
        name,
        city,
        country,
        alias,
        regions,
        coordinates,
        province,
        timezone,
        unlocs,
        code,
    ))
}