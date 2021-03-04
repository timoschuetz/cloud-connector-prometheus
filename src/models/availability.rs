pub enum Status {
    Available,
    Offline
}

pub struct URLAvailability {
    pub url: String,
    pub status: Status
}