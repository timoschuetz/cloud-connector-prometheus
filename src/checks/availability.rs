use std::sync::mpsc;
use std::net::Ipv4Addr;
use std::time::Duration;
use log::{info,warn,debug,error};

use crate::models;
use models::global::Instance;
use models::availability::{Status, URLAvailability};

pub fn check_availability(instance: Instance, tx: mpsc::Sender<URLAvailability>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}", instance.ip, instance.port);

        let mut result = URLAvailability {
            instance,
            status: Status::Offline,
        };

        // Initialize blocking client
        let client = reqwest::blocking::Client::builder().timeout(Duration::from_millis(500)).danger_accept_invalid_certs(true);

        match client.build() {
            Ok(client) => {
                match client.get(&request_url).send() {
                    Ok(data) => {
                        if data.status().is_success() {
                            result.status = Status::Available;
                        }
                        tx.send(result);
                    },
                    Err(e) => {
                        tx.send(result);
                        error!("error during request: {:?}", e);
                    }
                }
            },
            Err(e) => {
                tx.send(result);
                error!("error during client build: {:?}", e);
            }
        }

    });
}