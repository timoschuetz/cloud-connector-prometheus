#[macro_use]
extern crate serde_derive;

use std::sync::mpsc;

mod checks;
mod models;
mod metrics;

use env_logger::{
    Builder,
    Env,
};
use log::{info,warn,debug, error};
use prometheus_exporter::prometheus::{register_int_gauge_vec, IntGaugeVec};
use std::net::SocketAddr;
use confy::ConfyError;
use std::path::Path;

use metrics::subaccounts::{get_subaccount_metrics, SubaccountMonitor};
use metrics::connection_backends::{get_backend_connection_metrics, BackendConnectionMonitor};
use metrics::performance_backends::{get_backend_performance_metrics, BackendPerformanceMonitor};
use metrics::memory::{get_memory_metrics, MemoryMonitor};
use metrics::toptimeconsumers::{get_toptime_consumers_metrics, TopTimePerformanceMonitor};

use checks::availability::check_availability;
use checks::ha_role::check_haRole;
use checks::subaccounts::check_subaccounts;
use checks::connection_backends::check_backend_connections;
use checks::performance_backends::check_backend_performance;
use checks::memory::check_memory;
use checks::toptimeconsumers::check_consumers;
use models::availability::{Status, URLAvailability};
use models::ha_role::HARole;
use models::global::{MyConfig, Instance};

fn printer(element: String) {
    std::thread::spawn(move || {
        println!("{}", element);
    });
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { port: 9185, connectors: vec![Instance {version: 0, ip: "127.0.0.1".parse().unwrap(), port: 8443, username: "Administrator".to_string(), password: "manage".to_string()}] } }
}

fn main() {

    // Load application configuration 
    // TODO: Allow use of no configuration for sidecar mode
    let path_string = String::from("config.yml");
    let from_string = Path::new(&path_string);

    // Initialize empty config set containing instances
    let mut instances: Vec<Instance> = Vec::new();

    let cfg: Result<MyConfig, ConfyError> = confy::load_path(from_string);
    match cfg {
        Ok(config) => {
            // Read configuration file
            instances = config.connectors;
        },
        Err(e) => error!("error during loading of configuration file: {:?}", e),
    }

    // Setup logger with default level info so we can see the messages from
    // prometheus_exporter.
    Builder::from_env(Env::default().default_filter_or("info")).init();

    // Parse address used to bind exporter to.
    // TODO: add option in configuration to change port and ip
    let addr_raw = "0.0.0.0:9185";
    let addr: SocketAddr = addr_raw.parse().expect("can not parse listen addr");

    // Start exporter
    let exporter = prometheus_exporter::start(addr).expect("can not start exporter");

    info!("Starting up");

    // Initialize tunnel_state gauge
    let availabilty: IntGaugeVec = register_int_gauge_vec!("scc_available", "check if instance is available",&["instance"]).expect("can not create gauge scc_available");

    let subacc_monitor: SubaccountMonitor = metrics::subaccounts::get_subaccount_metrics();

    let connection_backend_monitor: BackendConnectionMonitor = metrics::connection_backends::get_backend_connection_metrics();

    let backend_performance_monitor: BackendPerformanceMonitor = metrics::performance_backends::get_backend_performance_metrics();

    let tt_consumers_monitor: TopTimePerformanceMonitor = metrics::toptimeconsumers::get_toptime_consumers_metrics();

    let memory_monitor: MemoryMonitor = metrics::memory::get_memory_metrics();

        loop {
            // Will block until a new request comes in.
            let _guard = exporter.wait_request();
            info!("Updating metrics");
    

            let (av_tx, av_rx) = mpsc::channel();
            let (ha_tx, ha_rx) = mpsc::channel();
            let (subaccount_tx, subaccount_rx) = mpsc::channel();
            let (backend_connection_tx, backend_connection_rx) = mpsc::channel();
            let (backend_performance_tx, backend_performance_rx) = mpsc::channel();
            let (memory_tx, memory_rx) = mpsc::channel();
            let (toptimeconsumers_tx, toptimeconsumers_rx) = mpsc::channel();
            let size = instances.len();

            // TODO: Pop non available instances out of check list

            for instance in &instances {
                check_availability(instance.ip.clone(), instance.port.clone(), av_tx.clone());
            }


            for line_res in av_rx.iter().take(size) {
                match line_res.status {
                    Status::Available => {
                        debug!("URL: {} - Available", line_res.url);
                        availabilty.with_label_values(&[&line_res.url]).set(1);
                    },
                    Status::Offline => {
                        debug!("URL: {} - Offline", line_res.url);
                        availabilty.with_label_values(&[&line_res.url]).set(0);
                    }
                }
            }


            for instance in &instances {
                check_haRole(instance.ip.clone(), instance.port.clone(), instance.username.clone(), instance.password.clone(), ha_tx.clone());
            }

            subacc_monitor.harole_master.reset();
            subacc_monitor.harole_shadow.reset();

            for line_res in ha_rx.iter().take(size) {
                match line_res.role {
                    HARole::Master => {
                        debug!("Server: {} - Master", line_res.url);
                        subacc_monitor.harole_master.with_label_values(&[&line_res.url]).set(1);
                    },
                    HARole::Shadow => {
                        debug!("Server: {} - Shadow", line_res.url);
                        subacc_monitor.harole_master.with_label_values(&[&line_res.url]).set(0);
                    },
                    HARole::Undefined => {
                        debug!("Server: {} - Undefined", line_res.url)
                    },
                }
            }


            for instance in &instances {
                check_subaccounts(instance.ip.clone(), instance.port.clone(), instance.username.clone(), instance.password.clone(), subaccount_tx.clone());
            }

            for line_res in subaccount_rx.iter().take(size) {
                match line_res.status {
                    models::subaccounts::SubaccountStatus::Exists => {
                        match line_res.payload {
                            Some(payload) => {
                                for subacc in payload.subaccounts {
                                    if subacc.tunnel.state == "Connected" {
                                        subacc_monitor.tunnel_state.with_label_values(&[&line_res.url, &subacc.subaccount]).set(1);
                                    } else {
                                        subacc_monitor.tunnel_state.with_label_values(&[&line_res.url, &subacc.subaccount]).set(0);
                                    }

                                    subacc_monitor.tunnel_connections.with_label_values(&[&line_res.url, &subacc.subaccount]).set(subacc.tunnel.connections);

                                    subacc_monitor.tunnel_certificate_expiration.with_label_values(&[&line_res.url, &subacc.subaccount]).set(subacc.tunnel.subaccount_certificate.not_after_time_stamp);

                                    subacc_monitor.tunnel_connected_since.with_label_values(&[&line_res.url, &subacc.subaccount]).set(subacc.tunnel.connected_since_time_stamp);

                                    for application in subacc.tunnel.application_connections {

                                        subacc_monitor.application_connections.with_label_values(&[&line_res.url, &subacc.subaccount, &application.name]).set(application.connection_count);

                                    }

                                    debug!("Got information about subaccount: {}", subacc.display_name);

                                }
                            },
                            None => {
                                debug!("Could not extract payload");
                            }
                        }
                    },
                    models::subaccounts::SubaccountStatus::Error => {
                        debug!("Error");
                    }
                }
                
            }

            for instance in &instances {
                check_backend_connections(instance.ip.clone(), instance.port.clone(), instance.username.clone(), instance.password.clone(), backend_connection_tx.clone());
            }

            for line_res in backend_connection_rx.iter().take(size) {
                match line_res.payload {
                    Some(data) => {
                        info!("Got Backend Connection DATA");
                        for subaccount in data.subaccounts {
                            for backend_connection in subaccount.backend_connections {
                                connection_backend_monitor.idle_backend_connection.with_label_values(&[&subaccount.region_host,&subaccount.subaccount,&backend_connection.virtual_backend,&backend_connection.internal_backend,&backend_connection.protocol]).set(backend_connection.idle);

                                connection_backend_monitor.active_backend_connection.with_label_values(&[&subaccount.region_host,&subaccount.subaccount,&backend_connection.virtual_backend,&backend_connection.internal_backend,&backend_connection.protocol]).set(backend_connection.active);
                            }
                        }
                    },
                    None => {
                        info!("Error");
                    }
                }
            }

            for instance in &instances {
                check_backend_performance(instance.ip.clone(), instance.port.clone(), instance.username.clone(), instance.password.clone(), backend_performance_tx.clone());
            }

            for line_res in backend_performance_rx.iter().take(size) {
                match line_res.payload {
                    Some(data) => {
                        for subaccount in data.subaccounts {
                            for backend in subaccount.backend_performance {
                                for bucket in backend.buckets {
                                    backend_performance_monitor.backend_performance_buckets.with_label_values(&[&subaccount.since_time,&subaccount.region_host,&subaccount.subaccount,&backend.virtual_host,&backend.virtual_port,&backend.protocol,&bucket.minimum_call_duration_ms.to_string()]).set(bucket.number_of_calls);
                                }
                            }
                        }
                    },
                    None => {
                        info!("Error");
                    }
                }
            }

            for instance in &instances {
                check_memory(instance.ip.clone(), instance.port.clone(), instance.username.clone(), instance.password.clone(), memory_tx.clone());
            }

            for line_res in memory_rx.iter().take(size) {
                match line_res.memory_info {
                    Some(data) => {
                            memory_monitor.memory_heap_total.with_label_values(&[&line_res.url.clone()]).set(data.cloud_connector_heap_kb.total);
                            memory_monitor.memory_heap_free.with_label_values(&[&line_res.url.clone()]).set(data.cloud_connector_heap_kb.free);
                            memory_monitor.memory_heap_used.with_label_values(&[&line_res.url.clone()]).set(data.cloud_connector_heap_kb.used);
                            memory_monitor.memory_physical_total.with_label_values(&[&line_res.url.clone()]).set(data.physical_kb.total);
                            memory_monitor.memory_physical_free.with_label_values(&[&line_res.url.clone()]).set(data.physical_kb.free);
                            memory_monitor.memory_physical_others.with_label_values(&[&line_res.url.clone()]).set(data.physical_kb.others);
                            memory_monitor.memory_physical_cloudconnector.with_label_values(&[&line_res.url.clone()]).set(data.physical_kb.cloud_connector);
                            memory_monitor.memory_virtual_total.with_label_values(&[&line_res.url.clone()]).set(data.virtual_kb.total);
                            memory_monitor.memory_virtual_free.with_label_values(&[&line_res.url.clone()]).set(data.virtual_kb.free);
                            memory_monitor.memory_virtual_others.with_label_values(&[&line_res.url.clone()]).set(data.virtual_kb.others);
                            memory_monitor.memory_virtual_cloudconnector.with_label_values(&[&line_res.url.clone()]).set(data.virtual_kb.cloud_connector);
                    },
                    None => {
                        info!("Error");
                    },
                }
            }

            for instance in &instances {
                check_consumers(instance.ip.clone(), instance.port.clone(), instance.username.clone(), instance.password.clone(), toptimeconsumers_tx.clone());
            }

            for line_res in toptimeconsumers_rx.iter().take(size) {
                match line_res.payload {
                    Some(data) => {
                        for subaccount in data.subaccounts {
                            for request in subaccount.requests {
                                tt_consumers_monitor.tt_cs_sent_bytes.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.sent_bytes);

                                tt_consumers_monitor.tt_cs_received_bytes.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.received_bytes);

                                tt_consumers_monitor.tt_cs_total_time.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.total_time);

                                tt_consumers_monitor.tt_cs_external_time.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.external_time);

                                tt_consumers_monitor.tt_cs_gen_sso_time.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.gen_sso_time);

                                tt_consumers_monitor.tt_cs_open_remote_time.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.open_remote_time);

                                tt_consumers_monitor.tt_cs_validate_sso_time.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.validate_sso_time);

                                tt_consumers_monitor.tt_cs_latency_time.with_label_values(&[&line_res.url.clone(), &subaccount.subaccount, &subaccount.region_host, &subaccount.location_id, &request.protocol, &request.virtual_backend, &request.internal_backend, &request.resource]).set(request.latency_time);
                            }
                        }
                    },
                    None => {
                        info!("Error");
                    }
                }
            }


        }

}
