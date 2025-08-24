mod os;
mod device;
mod cpu;
mod memory;
mod disk;
mod network;
mod tally;

#[cfg(feature = "http")]
pub mod http;

use std::time::Duration;
use serde_json::{Value, json};

use crate::models::SystemInfo;
use crate::utils::{parse_field_selector, filter_json_fields};

pub struct Collector {
    timeout: Duration,
    tally_enabled: bool,
    tally_http_endpoint: Option<String>,
}

impl Collector {
    pub fn new(timeout_ms: u64) -> Self {
        Self {
            timeout: Duration::from_millis(timeout_ms),
            tally_enabled: true,
            tally_http_endpoint: Some("127.0.0.1:9000".to_string()),
        }
    }
    
    pub fn disable_tally_detection(&mut self) {
        self.tally_enabled = false;
    }
    
    pub fn set_tally_http_endpoint(&mut self, endpoint: String) {
        self.tally_http_endpoint = Some(endpoint);
    }
    
    pub fn collect(&self) -> Value {
        let mut system_info = SystemInfo::default();
        
        // Collect OS information
        system_info.os = Some(os::collect_os_info());
        
        // Collect device information
        system_info.device = Some(device::collect_device_info());
        
        // Collect CPU information
        system_info.cpu = Some(cpu::collect_cpu_info());
        
        // Collect memory information
        system_info.memory = Some(memory::collect_memory_info());
        
        // Collect disk information
        system_info.disks = Some(disk::collect_disk_info());
        
        // Collect network information
        system_info.network = Some(network::collect_network_info());
        
        // Collect Tally information if enabled
        if self.tally_enabled {
            let tally_info = tally::collect_tally_info(self.timeout, self.tally_http_endpoint.clone());
            system_info.apps = Some(crate::models::AppsInfo {
                tally: Some(tally_info),
            });
        }
        
        // Convert to JSON Value
        serde_json::to_value(system_info).unwrap_or_else(|_| json!({}))
    }
    
    pub fn filter_fields(&self, value: Value, selector: String) -> Value {
        let fields = parse_field_selector(&selector);
        filter_json_fields(value, fields)
    }
    
    pub fn add_metadata(&self, value: &mut Value, duration_ms: u64) {
        if let Value::Object(obj) = value {
            let collector_info = json!({
                "name": "QuickSys",
                "version": env!("CARGO_PKG_VERSION"),
                "duration_ms": duration_ms
            });
            
            obj.insert("collector".to_string(), collector_info);
        }
    }
}