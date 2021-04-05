use crate::models;
use models::performance_toptime_consumers::TopTimeConsumers;
use std::sync::mpsc;
use log::{info,warn,debug};
use std::net::Ipv4Addr;

fn send_error(tx: mpsc::Sender<TopTimeConsumers>, url: String) {
    tx.send(TopTimeConsumers {
        url,
        payload: None,
    });
}

pub fn check_consumers(ip: Ipv4Addr, port: u16, username: String, password: String, tx: mpsc::Sender<TopTimeConsumers>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}/api/monitoring/performance/toptimeconsumers", ip, port);
        let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true)
        .build();

        let mut response = TopTimeConsumers {
            payload: None,
            url: format!("{}", ip),
        };

        match client {
            Ok(client) => {
                match client.get(&request_url).basic_auth(username, Some(password)).send() {
                    Ok(data) => {
                        match data.json::<models::performance_toptime_consumers::Root>() {
                            Ok(data) => {
                                response.payload = Some(data);
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