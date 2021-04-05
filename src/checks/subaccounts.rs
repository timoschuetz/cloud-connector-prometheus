
use crate::models;
use models::subaccounts::*;
use std::sync::mpsc;
use log::{info,warn,debug};
use std::net::Ipv4Addr;

fn send_error(tx: mpsc::Sender<SubaccountResponse>, url: String) {
    tx.send(SubaccountResponse {
        status: models::subaccounts::SubaccountStatus::Error,
        url,
        payload: None,
    });
}

pub fn check_subaccounts(ip: Ipv4Addr, port: u16, username: String, password: String, tx: mpsc::Sender<SubaccountResponse>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}:{}/api/monitoring/subaccounts", ip, port);
        let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true)
        .build();

        match client {
            Ok(client) => {
                match client.get(&request_url).basic_auth(username, Some(password)).send() {
                    Ok(data) => {
                        match data.json::<MonitSub>() {
                            Ok(data) => {
                                tx.send(SubaccountResponse {
                                    status: models::subaccounts::SubaccountStatus::Exists,
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