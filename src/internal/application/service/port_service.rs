use crate::internal::application::command::add_or_update::AddOrUpdate;
use crate::internal::domain::coordinates::Coordinates;
use crate::internal::domain::port::{add_or_update_port, Port};
use crate::internal::domain::port_id::PortId;
use crate::internal::domain::port_repository::{PortRepository, RepoAddError, RepoFindError, RepoUpdateError};

pub trait PortHandler {
    fn handle_add_or_update_port(&self, command: AddOrUpdate) -> Result<(), AddOrUpdateError>;
}

#[derive(Debug)]
pub enum AddError {
    DomainViolation(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum UpdateError {
    DomainViolation(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum AddOrUpdateError {
    DomainViolation(String),
    Unknown(String),
}

pub struct Service<T> {
    port_store: T,
}

impl <T: PortRepository> PortHandler for  Service<T> {
    pub fn handle_add_or_update_port(&self, command: AddOrUpdate) -> Result<(), AddOrUpdateError> {
        let result = self.port_store.find(PortId::new(command.port_id().to_string()));

        match result {
            Ok(_) => match self.handle_update_port(command) {
                Ok(_) => Ok(()),
                Err(e) => match e {
                    UpdateError::Unknown(e) => Err(AddOrUpdateError::Unknown(e)),
                    UpdateError::DomainViolation(e) => Err(AddOrUpdateError::DomainViolation(e.to_string()))
                }
            },
            Err(e) => match e {
                RepoFindError::NotFound => match self.handle_add_port(command) {
                    Ok(_) => Ok(()),
                    Err(e) => match e {
                        AddError::Unknown(e) => Err(AddOrUpdateError::Unknown(e)),
                        AddError::DomainViolation(e) => Err(AddOrUpdateError::DomainViolation(e.to_string()))
                    }
                }
                RepoFindError::Unknown(e) => Err(AddOrUpdateError::Unknown(e))
            }
        }
    }
}
impl <T: PortRepository> Service<T> {
    pub fn new(port_store: T) -> Self {
        Self { port_store }
    }

    pub fn handle_add_or_update_port(&self, command: AddOrUpdate) -> Result<(), AddOrUpdateError> {
        let result = self.port_store.find(PortId::new(command.port_id().to_string()));

        match result {
            Ok(_) => match self.handle_update_port(command) {
                Ok(_) => Ok(()),
                Err(e) => match e {
                    UpdateError::Unknown(e) => Err(AddOrUpdateError::Unknown(e)),
                    UpdateError::DomainViolation(e) => Err(AddOrUpdateError::DomainViolation(e.to_string()))
                }
            },
            Err(e) => match e {
                RepoFindError::NotFound => match self.handle_add_port(command) {
                    Ok(_) => Ok(()),
                    Err(e) => match e {
                        AddError::Unknown(e) => Err(AddOrUpdateError::Unknown(e)),
                        AddError::DomainViolation(e) => Err(AddOrUpdateError::DomainViolation(e.to_string()))
                    }
                }
                RepoFindError::Unknown(e) => Err(AddOrUpdateError::Unknown(e))
            }
        }
    }

    pub fn handle_add_port(&self, command: AddOrUpdate) -> Result<(), AddError> {
        let result = add_or_update_port(
            PortId::new(command.port_id().to_string()),
            command.name().to_string(),
            command.city().to_string(),
            command.country().to_string(),
            command.alias().to_vec(),
            command.regions().to_vec(),
            Coordinates::new(command.latitude(), command.longitude()),
            command.province().to_string(),
            command.timezone().to_string(),
            command.unlocs().to_vec(),
            command.code().to_string(),
        );

        match result {
            Ok(change) => {
                match self.port_store.add(change) {
                    Ok(_) => Ok(()),
                    Err(e) => match e {
                        RepoAddError::Unknown(e) => Err(AddError::Unknown(e))
                    }
                }
            }
            Err(e) => Err(AddError::DomainViolation(e.to_string()))
        }
    }

    pub fn handle_update_port(&self, command: AddOrUpdate) -> Result<(), UpdateError> {
        let result = add_or_update_port(
            PortId::new(command.port_id().to_string()),
            command.name().to_string(),
            command.city().to_string(),
            command.country().to_string(),
            command.alias().to_vec(),
            command.regions().to_vec(),
            Coordinates::new(command.latitude(), command.longitude()),
            command.province().to_string(),
            command.timezone().to_string(),
            command.unlocs().to_vec(),
            command.code().to_string(),
        );

        match result {
            Ok(change) => {
                match self.port_store.update(change) {
                    Ok(_) => Ok(()),
                    Err(e) => match e {
                        RepoUpdateError::Unknown(e) => Err(UpdateError::Unknown(e))
                    }
                }
            }
            Err(e) => Err(UpdateError::DomainViolation(e.to_string()))
        }
    }
}
