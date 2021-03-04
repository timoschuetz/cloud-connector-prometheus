use crate::models;
use models::haRole::HARole;
use std::sync::mpsc;

pub fn check_haRole(url: String, tx: mpsc::Sender<HARole>) {
    std::thread::spawn(move || {
        let request_url = format!("https://{}/api/v1/configuration/connector/haRole", url);
        let client = reqwest::blocking::Client::builder().danger_accept_invalid_certs(true)
        .build();

        match client {
            Ok(client) => {
                match client.get(&request_url).send() {
                    Ok(data) => println!("{}", data.status()),
                    Err(e) => println!("{:?}", e),
                }
            },
            Err(e) => println!("{:?}",e),
        }

    });
}