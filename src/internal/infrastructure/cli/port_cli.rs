use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::internal::application::command::add_or_update::AddOrUpdate;
use crate::internal::application::service::port_service::{AddOrUpdateError, PortHandler};
use crate::internal::infrastructure::cli::port::PortInformation;
use crate::internal::infrastructure::logger::base_logger::Logger;


pub struct PortCLI<T, U> {
    port_handler: T,
    logger: U,
}

impl<T: PortHandler, U: Logger> PortCLI<T, U> {
    pub fn new(port_handler: T, logger: U) -> Self {
        Self { port_handler, logger }
    }

    fn handle(&self, ports_file: String) {
        let file = File::open(ports_file).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let option: Option<PortInformation> = serde_json::from_str(&line.unwrap()).ok();
            let port_information = option.unwrap();

            let add_or_update_port = AddOrUpdate::new(
                port_information.id().to_string(),
                port_information.name().to_string(),
                port_information.city().to_string(),
                port_information.country().to_string(),
                port_information.alias().to_vec(),
                port_information.regions().to_vec(),
                port_information.coordinates()[1],
                port_information.coordinates()[0],
                port_information.timezone().to_string(),
                port_information.timezone().to_string(),
                port_information.unlocs().to_vec(),
                port_information.code().to_string()
            );

            // TODO
            self.port_handler.handle_add_or_update_port(add_or_update_port)
        }
    }
}

