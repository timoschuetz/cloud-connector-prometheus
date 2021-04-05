use prometheus_exporter::prometheus::{register_int_gauge_vec, IntGaugeVec};

pub struct SubaccountMonitor {
    pub tunnel_state: IntGaugeVec,
    pub harole_master: IntGaugeVec,
    pub harole_shadow: IntGaugeVec,
    pub tunnel_connections: IntGaugeVec,
    pub tunnel_certificate_expiration: IntGaugeVec,
    pub tunnel_connected_since: IntGaugeVec,
    pub application_connections: IntGaugeVec,
}

pub fn get_subaccount_metrics() -> SubaccountMonitor {

    // Initialize tunnel_state gauge
    let tunnel_state: IntGaugeVec = register_int_gauge_vec!("scc_subacc_tunnel_state", "check if tunnel is available",&["instance","subaccount"]).expect("can not create gauge scc_subacc_tunnel_state");

    // Initialize harole_master gauge
    let harole_master: IntGaugeVec = register_int_gauge_vec!("scc_harole_master", "check if HArole is master", &["instance"]).expect("can not create gauge scc_harole_master");

    // Initialize harole_shadow gauge
    let harole_shadow: IntGaugeVec = register_int_gauge_vec!("scc_harole_shadow", "check if HArole is master", &["instance"]).expect("can not create gauge scc_harole_shadow");

    // Initialize tunnel connection gauge
    let tunnel_connections: IntGaugeVec = register_int_gauge_vec!("scc_subacc_tunnel_connections", "check amount of active tunnel connections",&["instance","subaccount"]).expect("can not create gauge scc_subacc_tunnel_connection");

    // Initialize tunnel certificate expiration gauge
    let tunnel_certificate_expiration: IntGaugeVec = register_int_gauge_vec!("scc_subacc_tunnel_certificate_expiration", "check expiration timestamp of tunnel",&["instance","subaccount"]).expect("can not create gauge scc_subacc_tunnel_certificate_expiration");

    // Initialize tunnel connected since gauge
    let tunnel_connected_since: IntGaugeVec = register_int_gauge_vec!("scc_subacc_tunnel_connected_since", "check expiration timestamp of tunnel",&["instance","subaccount"]).expect("can not create gauge scc_subacc_tunnel_connected_since");

    // Initialize application connection gauge
    let application_connections: IntGaugeVec = register_int_gauge_vec!("scc_subacc_application_connections", "check amount of active application connections",&["instance","subaccount","application"]).expect("can not create gauge scc_subacc_tunnel_connection");

    return SubaccountMonitor {
        tunnel_state,
        harole_master,
        harole_shadow,
        tunnel_connections,
        tunnel_certificate_expiration,
        tunnel_connected_since,
        application_connections,
    }

}