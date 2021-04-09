use crate::models;
use models::global::Instance;

pub enum HARole {
    Master,
    Shadow,
    Undefined
}

pub struct HA {
    pub role: HARole,
    pub instance: Instance
}