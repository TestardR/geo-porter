use std::error::Error;

use super::port::Port;
use super::port_id::PortId;

pub trait PortFinder {
    fn find(&self, id: &PortId) -> Result<Port, Box<dyn Error>>;
}

pub trait PortAdder {
    fn add(&self, port: &Port) -> Result<(), Box<dyn Error>>;
}

pub trait PortUpdater {
    fn update(&self, port: &Port) -> Result<(), Box<dyn Error>>;
}

