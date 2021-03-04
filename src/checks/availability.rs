use std::sync::mpsc;

use crate::models;
use models::availability::{Status, URLAvailability};

pub fn check_availability(url: String, tx: mpsc::Sender<URLAvailability>) {
    std::thread::spawn(move || {
        let request = reqwest::blocking::get(&url);
        let mut result = URLAvailability {
            url: url,
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