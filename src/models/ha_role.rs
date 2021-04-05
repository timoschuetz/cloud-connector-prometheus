pub enum HARole {
    Master,
    Shadow,
    Undefined
}

pub struct HA {
    pub role: HARole,
    pub url: String
}