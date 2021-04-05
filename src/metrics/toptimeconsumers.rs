use prometheus_exporter::prometheus::{register_int_gauge_vec, IntGaugeVec};

pub struct TopTimePerformanceMonitor {
    pub tt_cs_sent_bytes: IntGaugeVec,
    pub tt_cs_received_bytes: IntGaugeVec,
    pub tt_cs_total_time: IntGaugeVec,
    pub tt_cs_external_time: IntGaugeVec,
    pub tt_cs_gen_sso_time: IntGaugeVec,
    pub tt_cs_open_remote_time: IntGaugeVec,
    pub tt_cs_validate_sso_time: IntGaugeVec,
    pub tt_cs_latency_time: IntGaugeVec,
}

pub fn get_toptime_consumers_metrics() -> TopTimePerformanceMonitor {

    // Initialize scc_tt_performance_sent_bytes gauge
    let tt_cs_sent_bytes: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_sent_bytes", "performance toptimeconsumers sent bytes",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_sent_bytes");

    // Initialize scc_tt_performance_received_bytes gauge
    let tt_cs_received_bytes: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_received_bytes", "performance toptimeconsumers received bytes",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_received_bytes");

    // Initialize scc_tt_performance_total_time gauge
    let tt_cs_total_time: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_total_time", "performance toptimeconsumers total time",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_total_time");

    // Initialize scc_tt_performance_external_time gauge
    let tt_cs_external_time: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_external_time", "performance toptimeconsumers external time",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_external_time");

    // Initialize scc_tt_performance_gen_sso_time gauge
    let tt_cs_gen_sso_time: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_gen_sso_time", "performance toptimeconsumers gen sso time",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_gen_sso_time");

    // Initialize scc_tt_performance_open_remote_time gauge
    let tt_cs_open_remote_time: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_open_remote_time", "performance toptimeconsumers open remote time",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_open_remote_time");

    // Initialize scc_tt_performance_validate_sso_time gauge
    let tt_cs_validate_sso_time: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_validate_sso_time", "performance toptimeconsumers validate sso time",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_validate_sso_time");

    // Initialize scc_tt_performance_latency_time gauge
    let tt_cs_latency_time: IntGaugeVec = register_int_gauge_vec!("scc_tt_performance_latency_time", "performance toptimeconsumers latency time",&["instance", "subaccount", "region_host", "location_id", "protocol", "virtual_backend", "internal_backend", "resource"]).expect("can not create gauge scc_tt_performance_latency_time");

    return TopTimePerformanceMonitor {
        tt_cs_sent_bytes,
        tt_cs_received_bytes,
        tt_cs_total_time,
        tt_cs_external_time,
        tt_cs_gen_sso_time,
        tt_cs_open_remote_time,
        tt_cs_validate_sso_time,
        tt_cs_latency_time,
    }

}
