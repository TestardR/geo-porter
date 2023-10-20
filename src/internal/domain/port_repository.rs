use super::port::Port;
use super::port_id::PortId;

#[derive(Debug)]
pub enum RepoFindError {
    Unknown(String),
    NotFound,
}

#[derive(Debug)]
pub enum RepoAddError {
    Unknown(String),
}

#[derive(Debug)]
pub enum UpdateError {
    Unknown(String),
}

pub trait PortRepository {
    fn find(&self, id: &PortId) -> Result<Port, RepoFindError>;
    fn add(&self, port: &Port) -> Result<(), RepoAddError>;
    fn update(&self, port: &Port) -> Result<(), UpdateError>;
}
