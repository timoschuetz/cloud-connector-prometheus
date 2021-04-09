use crate::models;
use models::ha_role::{HARole, HA};
use models::global::Instance;
use log::{info,warn,debug, error};
use std::sync::mpsc;
use std::net::Ipv4Addr;

pub fn check_haRole(instance: Instance, tx: mpsc::Sender<HA>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}/api/v1/configuration/connector/haRole", instance.ip, instance.port);
        let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true)
        .build();

        // Clone username and password because instance will be moved by assining it to response
        let username = instance.username.clone();
        let password = instance.password.clone();

        let mut response = HA {
            role: HARole::Undefined,
            instance,
        };

        match client {
            Ok(client) => {
                match client.get(&request_url).basic_auth(username, Some(password)).send() {
                    Ok(data) => {
                        match data.text() {
                            Ok(text) => {
                                if text.contains("master") {
                                    response.role = HARole::Master;
                                } else if text.contains("shadow") {
                                    response.role = HARole::Shadow
                                }
                                tx.send(response);
                            },
                            Err(e) => {
                                // TODO: Remove/Rewrite
                                println!("{:?}", e);
                                tx.send(response);
                            },
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
                println!("{:?}",e);
                tx.send(response);
            },
        }

    });
}