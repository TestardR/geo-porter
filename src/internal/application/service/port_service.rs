use crate::internal::application::command::add_or_update::AddOrUpdate;
use crate::internal::domain::coordinates::Coordinates;
use crate::internal::domain::port::{add_or_update_port, Port};
use crate::internal::domain::port_id::PortId;
use crate::internal::domain::port_repository::{PortRepository, RepoFindError};

#[derive(Debug)]
pub enum AddError {
    DomainViolation(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum UpdateError {
    Unknown(String),
}

#[derive(Debug)]
enum AddOrUpdateError {
    AddError,
    UpdateError,
}

pub struct Service<T> {
    port_store: T,
}

impl<T: PortRepository> Service<T> {
    pub fn new(port_store: T) -> Self {
        Self { port_store }
    }

    pub fn handle_add_or_update_port(&self, command: AddOrUpdate) -> Result<(), AddOrUpdateError> {
        let result = self.port_store.find(&PortId::new(command.port_id().to_string()));

        match result {
            Ok(port) => self.handle_update_port(command, port),
            Err(e) => match e {
                RepoFindError::NotFound => {
                    let &alias = command.alias();
                    let &regions = command.regions();
                    let &unlocs = command.unlocs();

                    let result = self.handle_add_port(command);
                    // TODO
                }
                RepoFindError::Unknown(e) => Err(e)
            }
        }

    }

    pub fn handle_add_port(&self, command: AddOrUpdate) -> Result<(), AddError> {
        let &alias = command.alias();
        let &regions = command.regions();
        let &unlocs = command.unlocs();

        let result = add_or_update_port(
            PortId::new(command.port_id().to_string()),
            command.name().to_string(),
            command.city().to_string(),
            command.country().to_string(),
            alias,
            regions,
            Coordinates::new(command.latitude(), command.longitude()),
            command.province().to_string(),
            command.timezone().to_string(),
            unlocs,
            command.code().to_string(),
        );

        Ok(())
    }

    pub fn handle_update_port(&self, command: AddOrUpdate, port: Port) -> Result<(), UpdateError> {
        let &alias = command.alias();
        let &regions = command.regions();
        let &unlocs = command.unlocs();

        let result = add_or_update_port(
            PortId::new(command.port_id().to_string()),
            command.name().to_string(),
            command.city().to_string(),
            command.country().to_string(),
            alias,
            regions,
            Coordinates::new(command.latitude(), command.longitude()),
            command.province().to_string(),
            command.timezone().to_string(),
            unlocs,
            command.code().to_string(),
        );

        Ok(())
    }
}
