use std::collections::HashMap;
use std::error::Error;
use std::sync::{Mutex, MutexGuard};
use crate::internal::domain::add_or_update_port_change::AddOrUpdatePortChange;

use crate::internal::domain::port_id::PortId;
use crate::internal::domain::port::Port;
use crate::internal::domain::port_repository::{RepoAddError, RepoFindError};

pub struct PortStore {
    db: Mutex<HashMap<PortId, Port>>
}

impl PortStore {
    pub fn new(db: Mutex<HashMap<PortId, Port>>) -> Self {
        Self { db }
    }

    fn find(&self, id: PortId) -> Result<Port, RepoFindError> {
        let db = match self.db.lock() {
            Ok(db) => {
            },
            Err(e) => Err(RepoFindError::Unknown(e.to_string()))
        };

        Err(RepoFindError::NotFound)
    }

    fn add(&self, change: AddOrUpdatePortChange) -> Result<(), RepoAddError> {
        Ok(())
    }

    fn update(&self, change: AddOrUpdatePortChange) -> Result<(), RepoFindError> {
        Ok(())
    }
}