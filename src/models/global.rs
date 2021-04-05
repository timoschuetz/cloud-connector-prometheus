use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize)]
pub struct MyConfig {
    pub port: u16,
    pub connectors: Vec<Instance>,
}

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub version: u8,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub username: String,
    pub password: String,
}