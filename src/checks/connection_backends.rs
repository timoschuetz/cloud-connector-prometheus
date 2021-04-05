use crate::models;
use models::connection_backends::*;
use std::sync::mpsc;
use log::{info,warn,debug};
use std::net::Ipv4Addr;

fn send_error(tx: mpsc::Sender<BackendConnections>, url: String) {
    tx.send(BackendConnections {
        url,
        payload: None,
    });
}

pub fn check_backend_connections(ip: Ipv4Addr, port: u16, username: String, password: String, tx: mpsc::Sender<BackendConnections>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}/api/monitoring/connections/backends", ip, port);
        let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true)
        .build();

        match client {
            Ok(client) => {
                match client.get(&request_url).basic_auth(username, Some(password)).send() {
                    Ok(data) => {
                        match data.json::<BackendConnectionsRoot>() {
                            Ok(data) => {
                                tx.send(BackendConnections {
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