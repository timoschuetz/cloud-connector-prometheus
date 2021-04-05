use prometheus_exporter::prometheus::{register_int_gauge_vec, IntGaugeVec};

pub struct MemoryMonitor {
    pub memory_heap_total: IntGaugeVec,
    pub memory_heap_free: IntGaugeVec,
    pub memory_heap_used: IntGaugeVec,
    pub memory_physical_total: IntGaugeVec,
    pub memory_physical_cloudconnector: IntGaugeVec,
    pub memory_physical_others: IntGaugeVec,
    pub memory_physical_free: IntGaugeVec,
    pub memory_virtual_total: IntGaugeVec,
    pub memory_virtual_cloudconnector: IntGaugeVec,
    pub memory_virtual_others: IntGaugeVec,
    pub memory_virtual_free: IntGaugeVec,
}

pub fn get_memory_metrics() -> MemoryMonitor {

    // Initialize memory_heap_total gauge
    let memory_heap_total: IntGaugeVec = register_int_gauge_vec!("scc_memory_heap_total", "total heap memory",&["instance"]).expect("can not create gauge scc_memory_heap_total");

    // Initialize memory_heap_free gauge
    let memory_heap_free: IntGaugeVec = register_int_gauge_vec!("scc_memory_heap_free", "free heap memory",&["instance"]).expect("can not create gauge scc_memory_heap_free");

    // Initialize memory_heap_used gauge
    let memory_heap_used: IntGaugeVec = register_int_gauge_vec!("scc_memory_heap_used", "used heap memory",&["instance"]).expect("can not create gauge scc_memory_heap_used");

    // Initialize memory_physical_total gauge
    let memory_physical_total: IntGaugeVec = register_int_gauge_vec!("scc_memory_physical_total", "total physical memory",&["instance"]).expect("can not create gauge scc_memory_heap_total");

    // Initialize memory_physical_cloudconnector gauge
    let memory_physical_cloudconnector: IntGaugeVec = register_int_gauge_vec!("scc_memory_physical_cloudconnector", "total physical cloud connector memory",&["instance"]).expect("can not create gauge scc_memory_physical_cloudconnector");

     // Initialize memory_physical_others gauge
     let memory_physical_others: IntGaugeVec = register_int_gauge_vec!("scc_memory_physical_others", "total physical others memory",&["instance"]).expect("can not create gauge scc_memory_physical_others");

     // Initialize memory_physical_free gauge
     let memory_physical_free: IntGaugeVec = register_int_gauge_vec!("scc_memory_physical_free", "total physical free memory",&["instance"]).expect("can not create gauge scc_memory_physical_free");

     // Initialize memory_virtual_total gauge
    let memory_virtual_total: IntGaugeVec = register_int_gauge_vec!("scc_memory_virtual_total", "total virtual memory",&["instance"]).expect("can not create gauge scc_memory_heap_total");

    // Initialize memory_virtual_cloudconnector gauge
    let memory_virtual_cloudconnector: IntGaugeVec = register_int_gauge_vec!("scc_memory_virtual_cloudconnector", "total virtual cloud connector memory",&["instance"]).expect("can not create gauge scc_memory_virtual_cloudconnector");

     // Initialize memory_virtual_others gauge
     let memory_virtual_others: IntGaugeVec = register_int_gauge_vec!("scc_memory_virtual_others", "total virtual others memory",&["instance"]).expect("can not create gauge scc_memory_virtual_others");

     // Initialize memory_virtual_free gauge
     let memory_virtual_free: IntGaugeVec = register_int_gauge_vec!("scc_memory_virtual_free", "total virtual free memory",&["instance"]).expect("can not create gauge scc_memory_virtual_free");

    return MemoryMonitor {
        memory_heap_total,
        memory_heap_free,
        memory_heap_used,
        memory_physical_total,
        memory_physical_free,
        memory_physical_others,
        memory_physical_cloudconnector,
        memory_virtual_total,
        memory_virtual_free,
        memory_virtual_others,
        memory_virtual_cloudconnector,
    }

}
