use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SystemInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector: Option<CollectorInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<OsInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<DeviceInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<CpuInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryInfo>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<DiskInfo>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<NetworkInfo>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<AppsInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CollectorInfo {
    pub name: String,
    pub version: String,
    pub duration_ms: u64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OsInfo {
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DeviceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CpuInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_cores: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_cores: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frequency_mhz: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MemoryInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_mb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_mb: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DiskInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_letter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fs_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_gb: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_gb: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NetworkInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppsInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tally: Option<TallyInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TallyInfo {
    pub installed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_source: Option<String>,
}