pub struct AddOrUpdate {
    port_id: String,
    name: String,
    city: String,
    country: String,
    alias: Vec<String>,
    regions: Vec<String>,
    latitude: f64,
    longitude: f64,
    province: String,
    timezone: String,
    unlocs: Vec<String>,
    code: String,
}

impl AddOrUpdate {
    pub fn new(
        port_id: String,
        name: String,
        city: String,
        country: String,
        alias: Vec<String>,
        regions: Vec<String>,
        latitude: f64,
        longitude: f64,
        province: String,
        timezone: String,
        unlocs: Vec<String>,
        code: String) -> Self {
        Self {
            port_id,
            name,
            city,
            country,
            alias,
            regions,
            latitude,
            longitude,
            province,
            timezone,
            unlocs,
            code
        }
    }

    pub fn port_id(&self) -> &str {
        &self.port_id
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

    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    pub fn longitude(&self) -> f64 {
        self.longitude
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