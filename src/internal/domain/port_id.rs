#[derive(Clone, Hash, Eq, PartialEq)]
pub struct PortId {
    id: String
}


impl PortId {
    pub fn new(id: String) -> Self {
        Self {id}
    }

    pub fn id(&self) -> &str {
        &self.id
    }
 }
