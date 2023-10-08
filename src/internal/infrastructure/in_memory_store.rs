use std::collections::HashMap;
use std::error::Error;
use std::sync::{Mutex, MutexGuard};

use crate::internal::domain::port_id::PortId;
use crate::internal::domain::port::Port;

pub struct PortStore {
    db: Mutex<HashMap<PortId, Port>>
}

impl PortStore {
    pub fn new(db: Mutex<HashMap<PortId, Port>>) -> Self {
        Self { db }
    }

    fn find(&self, id: &PortId) -> Result<Port, Box<dyn Error>> {
        let db = match self.db.lock() {
            Ok(db) => db,
            Err(e) => Err(e)
        };
    }

    fn add(&self, port: &Port) -> Result<(), Box<dyn Error>> {

    }

    fn update(&self, port: &Port) -> Result<(), Box<dyn Error>> {

    }
}