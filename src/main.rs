use std::sync::mpsc;

mod checks;
mod models;

use checks::availability::check_availability;
use checks::haRole::check_haRole;
use models::availability::{Status, URLAvailability};
use models::haRole::HARole;

fn printer(element: String) {
    std::thread::spawn(move || {
        println!("{}", element);
    });
}

fn main() {
    println!("Hello, world!");
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let urls = vec!["https://www.rust-lang.org", "https://www.google.com", "https://www.timoschuetz.com", "https://nx"];

    let (av_tx, av_rx) = mpsc::channel();
    let (ha_tx, ha_rx) = mpsc::channel();
    let size = urls.len();

    for element in arr {
        printer(element.to_string());
    }

    for url in urls {
        check_availability(url.clone().to_string(), av_tx.clone());
        check_haRole(url.clone().to_string(), ha_tx.clone());
    }

    for line_res in av_rx.iter().take(size) {
        match line_res.status {
            Status::Available => {
                println!("URL: {} - Available", line_res.url);
            },
            Status::Offline => {
                println!("URL: {} - Offline", line_res.url);
            }
        }
    }



    
}
