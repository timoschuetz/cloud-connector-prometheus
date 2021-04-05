use prometheus_exporter::prometheus::{register_int_gauge_vec, IntGaugeVec};

pub struct BackendPerformanceMonitor {
    pub backend_performance_buckets: IntGaugeVec,
}

pub fn get_backend_performance_metrics() -> BackendPerformanceMonitor {

    // Initialize idle_backend_connection gauge
    let backend_performance_buckets: IntGaugeVec = register_int_gauge_vec!("scc_backend_performance_buckets", "backend performance buckets",&["since_time","region_host","subaccount","virtual_host","virtual_port","protocol","minimum_call_duration_ms"]).expect("can not create gauge scc_backend_performance_buckets");

    return BackendPerformanceMonitor {
        backend_performance_buckets,
    }

}
