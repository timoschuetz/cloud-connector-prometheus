use crate::models;
use models::memory::Memory;
use std::sync::mpsc;
use log::{info,warn,debug};
use std::net::Ipv4Addr;

fn send_error(tx: mpsc::Sender<Memory>, url: String) {
    tx.send(Memory {
        url,
        memory_info: None,
    });
}

pub fn check_memory(ip: Ipv4Addr, port: u16, username: String, password: String, tx: mpsc::Sender<Memory>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}/api/monitoring/memory", ip, port);
        let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true)
        .build();

        let mut response = Memory {
            memory_info: None,
            url: format!("{}", ip),
        };

        match client {
            Ok(client) => {
                match client.get(&request_url).basic_auth(username, Some(password)).send() {
                    Ok(data) => {
                        match data.json::<models::memory::Root>() {
                            Ok(data) => {
                                response.memory_info = Some(data);
                                tx.send(response);
                            },
                            Err(e) => {
                                send_error(tx, format!("{}", ip));
                                warn!("Error during conversion: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        // TODO: Remove/Rewrite
                        println!("{:?}", e);
                        tx.send(response);
                    }
                }
            },
            Err(e) => {
                // TODO: Remove/Rewrite
                println!("{:?}",e);
                tx.send(response);
            },
        }

    });
}