use crate::models;
use models::performance_backends::*;
use std::sync::mpsc;
use log::{info,warn,debug};
use std::net::Ipv4Addr;

fn send_error(tx: mpsc::Sender<MBackendPerformance>, url: String) {
    tx.send(MBackendPerformance {
        url,
        payload: None,
    });
}

pub fn check_backend_performance(ip: Ipv4Addr, port: u16, username: String, password: String, tx: mpsc::Sender<MBackendPerformance>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}/api/monitoring/performance/backends", ip, port);
        let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true)
        .build();

        match client {
            Ok(client) => {
                match client.get(&request_url).basic_auth(username, Some(password)).send() {
                    Ok(data) => {
                        match data.json::<BackendPerformanceRoot>() {
                            Ok(data) => {
                                tx.send(MBackendPerformance {
                                    url: format!("{}", ip),
                                    payload: Some(data),
                                });
                            },
                            Err(e) => {
                                send_error(tx, format!("{}", ip));
                                warn!("Error during conversion: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        send_error(tx, format!("{}", ip));
                        warn!("{:?}", e);
                    }
                }
            },
            Err(e) => {
                send_error(tx, format!("{}", ip));
                warn!("{:?}",e);
            },
        }

    });
}