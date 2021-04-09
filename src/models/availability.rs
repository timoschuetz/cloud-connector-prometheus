use crate::models;
use models::global::Instance;

pub enum Status {
    Available,
    Offline
}

pub struct URLAvailability {
    pub instance: Instance,
    pub status: Status
}