use crate::models::CpuInfo;
use windows::Win32::System::SystemInformation;
use std::mem::zeroed;

pub fn collect_cpu_info() -> CpuInfo {
    let mut cpu_info = CpuInfo::default();
    
    // Get CPU information from WMI
    if let Ok(wmi_con) = wmi::WMIConnection::new() {
        // Query Win32_Processor for CPU details
        if let Ok(results) = wmi_con.query::<serde_json::Value>("SELECT Name, NumberOfCores, NumberOfLogicalProcessors, MaxClockSpeed FROM Win32_Processor") {
            if let Some(processor) = results.get(0) {
                if let Some(name) = processor.get("Name").and_then(|v| v.as_str()) {
                    cpu_info.name = Some(name.trim().to_string());
                }
                
                if let Some(cores) = processor.get("NumberOfCores").and_then(|v| v.as_u64()) {
                    cpu_info.physical_cores = Some(cores as u32);
                }
                
                if let Some(logical_cores) = processor.get("NumberOfLogicalProcessors").and_then(|v| v.as_u64()) {
                    cpu_info.logical_cores = Some(logical_cores as u32);
                }
                
                if let Some(max_speed) = processor.get("MaxClockSpeed").and_then(|v| v.as_u64()) {
                    cpu_info.max_frequency_mhz = Some(max_speed as u32);
                }
            }
        }
    }
    
    // Fallback for logical processors count if WMI failed
    if cpu_info.logical_cores.is_none() {
        unsafe {
            let mut system_info: SystemInformation::SYSTEM_INFO = zeroed();
            SystemInformation::GetSystemInfo(&mut system_info);
            cpu_info.logical_cores = Some(system_info.dwNumberOfProcessors as u32);
        }
    }
    
    cpu_info
}