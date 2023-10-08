use crate::internal::{
    domain::{
        error::DomainError,
        port::Port,
        port_id::PortId,
        coordinates::Coordinates,
        port_validator::Violations,
        port_repository::{
            PortAdder,
            PortFinder,
            PortUpdater,
        },
    },
    application::{
        command::add_or_update::AddOrUpdate,
        shared::error_handler::ErrorHandler,
    },
};

trait PortFindAddUpdater: PortFinder + PortAdder + PortUpdater {}

trait PortValidator {
    fn validate(&self, port: Port) -> Result<(), Violations>;
}

pub struct Service<'a> {
    port_store: &'a dyn PortFindAddUpdater,
    port_validator: &'a dyn PortValidator,
}

impl<'a> Service<'a> {
    pub fn new(
        port_store: &'a dyn PortFindAddUpdater,
        port_validator: &'a dyn PortValidator,
    ) -> Self {
        Self {
            port_store,
            port_validator,
        }
    }

    pub fn handle_add_or_update_port(&self, command: AddOrUpdate) -> Result<(), DomainError> {
        match self.port_store.find(&PortId::new(command.port_id().to_string())) {
            Ok(port) => self.handle_update_port(command, port),
            Err(e) => {}
        }
    }

    pub fn handle_add_port(&self, command: AddOrUpdate) -> Result<(), DomainError> {
        let &alias = command.alias();
        let &regions = command.regions();
        let &unlocs = command.unlocs();

        let port = Port::new(
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

        match self.port_validator.validate(port) {
            Err(e) => {
                let port_id = command.port_id();
                Err(ErrorHandler::application_error(&format!("Could not add port with id {port_id}"), Some(Box::new(e))))
            }
            Ok(_) => {
                match self.port_store.add(&port) {
                    Err(e) => {
                        let port_id = command.port_id();
                        Err(ErrorHandler::application_error(&format!("Could not update port with id {port_id}"), Some(e)))
                    }
                    Ok(_) => Ok(())
                }
            }
        }
    }

    pub fn handle_update_port(&self, command: AddOrUpdate, port: Port) -> Result<(), DomainError> {
        let &alias = command.alias();
        let &regions = command.regions();
        let &unlocs = command.unlocs();

        let update_port_change = port.update_port_change(
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

        match self.port_validator.validate(update_port_change) {
            Err(e) => {
                let port_id = command.port_id();
                Err(ErrorHandler::application_error(&format!("Could not update port with id {port_id}"), Some(Box::new(e))))
            }
            Ok(_) => {
                match self.port_store.update(&update_port_change) {
                    Err(e) => {
                        let port_id = command.port_id();
                        Err(ErrorHandler::application_error(&format!("Could not update port with id {port_id}"), Some(e)))
                    }
                    Ok(_) => Ok(())
                }
            }
        }
    }
}
