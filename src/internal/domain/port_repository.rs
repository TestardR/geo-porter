use super::port::Port;
use super::port_id::PortId;

pub trait PortFinder {
    fn find(&self, id: PortId) -> Port;
}
pub trait PortAdder {
    fn add(&self, port: Port);
}
pub trait PortUpdater {
    fn update(&self, port: Port);
}

