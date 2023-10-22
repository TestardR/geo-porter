use crate::internal::domain::add_or_update_port_change::AddOrUpdatePortChange;
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
pub enum RepoUpdateError {
    Unknown(String),
}

pub trait PortRepository {
    fn find(&self, id: PortId) -> Result<Port, RepoFindError>;
    fn add(&self, change: AddOrUpdatePortChange) -> Result<(), RepoAddError>;
    fn update(&self, change: AddOrUpdatePortChange) -> Result<(), RepoUpdateError>;
}
