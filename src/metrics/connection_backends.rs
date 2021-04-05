use prometheus_exporter::prometheus::{register_int_gauge_vec, IntGaugeVec};

pub struct BackendConnectionMonitor {
    pub idle_backend_connection: IntGaugeVec,
    pub active_backend_connection: IntGaugeVec,
}

pub fn get_backend_connection_metrics() -> BackendConnectionMonitor {

    // Initialize idle_backend_connection gauge
    let idle_backend_connection: IntGaugeVec = register_int_gauge_vec!("scc_idle_backend_connection", "idle backend connection",&["region_host","subaccount","virtual_backend","internal_backend","protocol"]).expect("can not create gauge scc_idle_backend_connection");

    // Initialize active_backend_connection gauge
    let active_backend_connection: IntGaugeVec = register_int_gauge_vec!("scc_active_backend_connection", "active backend connection",&["region_host","subaccount","virtual_backend","internal_backend","protocol"]).expect("can not create gauge scc_active_backend_connection");

    return BackendConnectionMonitor {
        idle_backend_connection,
        active_backend_connection,
    }

}
