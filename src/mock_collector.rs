//! Mock collector implementation for non-Windows platforms
//! Provides stub implementations that return placeholder data

use serde_json::{Value, json};
use std::time::Duration;

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
        // Return mock data for non-Windows platforms
        json!({
            "os": {
                "family": "Mock OS",
                "edition": "Development",
                "version": "1.0",
                "build": "dev",
                "arch": "x86_64"
            },
            "device": {
                "hostname": "mock-device",
                "manufacturer": "Mock Manufacturer",
                "model": "Development Model",
                "uuid": "00000000-0000-0000-0000-000000000000"
            },
            "cpu": {
                "name": "Mock CPU",
                "physical_cores": 4,
                "logical_cores": 8,
                "max_frequency_mhz": 3000
            },
            "memory": {
                "installed_mb": 16384,
                "available_mb": 8192
            },
            "disks": [
                {
                    "drive_letter": "C:",
                    "fs_type": "NTFS",
                    "total_gb": 500.0,
                    "free_gb": 250.0
                }
            ],
            "network": [
                {
                    "interface_name": "Mock Ethernet",
                    "mac_address": "00:00:00:00:00:00",
                    "ipv4_addresses": ["192.168.1.100"],
                    "ipv6_addresses": ["fe80::0000:0000:0000:0000"]
                }
            ],
            "apps": {
                "tally": {
                    "installed": self.tally_enabled,
                    "variant": "TallyPrime",
                    "version": "3.0.1",
                    "detection_source": "mock"
                }
            }
        })
    }
    
    pub fn filter_fields(&self, value: Value, selector: String) -> Value {
        // Use the real implementation from utils
        let fields = crate::utils::parse_field_selector(&selector);
        crate::utils::filter_json_fields(value, fields)
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