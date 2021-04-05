use std::sync::mpsc;
use std::net::Ipv4Addr;

use crate::models;
use models::availability::{Status, URLAvailability};

pub fn check_availability(ip: Ipv4Addr, port: u16, tx: mpsc::Sender<URLAvailability>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}", ip, port);
        let request = reqwest::blocking::get(&request_url);
        let mut result = URLAvailability {
            url: format!("{}:{}", ip, port),
            status: Status::Offline,
        };
        match request {
            Ok(data) => {
                if data.status().is_success() {
                    result.status = Status::Available;
                }
                tx.send(result);
            },
            Err(e) => {
                tx.send(result);
            },
        };

    });
}