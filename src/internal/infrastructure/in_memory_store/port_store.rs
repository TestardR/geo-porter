use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::internal::domain::add_or_update_port_change::AddOrUpdatePortChange;
use crate::internal::domain::coordinates::Coordinates;

use crate::internal::domain::port_id::PortId;
use crate::internal::domain::port::Port;
use crate::internal::domain::port_repository::{RepoAddError, RepoFindError};

pub struct PortStore {
    db: Arc<Mutex<HashMap<PortId, Port>>>,
}

impl PortStore {
    pub fn new(db: Arc<Mutex<HashMap<PortId, Port>>>) -> Self {
        Self { db }
    }
    // Arc::new(Mutex::new(HashMap::new()));
    fn find(&self, port_id: PortId) -> Result<Port, RepoFindError> {
        let db = self.db.lock().unwrap();
        match  db.get(&port_id) {
            Some(port) => Ok(port.clone()),
            None => Err(RepoFindError::NotFound)
        }
    }

    fn add(&mut self, change: AddOrUpdatePortChange) -> Result<(), RepoAddError> {
        let port_id = change.id().id().clone();
        let port = Port::new(
            PortId::new(port_id.to_string()),
            change.name().to_string(),
        change.city().to_string(),
            change.country().to_string(),
            change.alias().to_vec(),
            change.regions().to_vec(),
            Coordinates::new(change.coordinates().latitude(), change.coordinates().longitude()),
            change.province().to_string(),
            change.timezone().to_string(),
            change.unlocs().to_vec(),
            change.code().to_string()
        );

        self.db.lock().unwrap().insert(PortId::new(port_id.to_string()), port);

        Ok(())
    }

    fn update(&mut self, change: AddOrUpdatePortChange) -> Result<(), RepoFindError> {
        let port_id = change.id().id().clone();
        let port = Port::new(
            PortId::new(port_id.to_string()),
            change.name().to_string(),
            change.city().to_string(),
            change.country().to_string(),
            change.alias().to_vec(),
            change.regions().to_vec(),
            Coordinates::new(change.coordinates().latitude(), change.coordinates().longitude()),
            change.province().to_string(),
            change.timezone().to_string(),
            change.unlocs().to_vec(),
            change.code().to_string()
        );

        self.db.lock().unwrap().insert(PortId::new(port_id.to_string()), port);

        Ok(())
    }
}