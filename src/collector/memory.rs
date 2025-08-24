use crate::models::MemoryInfo;
use windows::Win32::System::SystemInformation;
use std::mem::zeroed;

pub fn collect_memory_info() -> MemoryInfo {
    let mut memory_info = MemoryInfo::default();
    
    unsafe {
        // Get memory status information
        let mut status: SystemInformation::MEMORYSTATUSEX = zeroed();
        status.dwLength = std::mem::size_of::<SystemInformation::MEMORYSTATUSEX>() as u32;
        
        if SystemInformation::GlobalMemoryStatusEx(&mut status).is_ok() {
            // Convert bytes to megabytes
            memory_info.installed_mb = Some(status.ullTotalPhys / (1024 * 1024));
            memory_info.available_mb = Some(status.ullAvailPhys / (1024 * 1024));
        }
    }
    
    // Fallback to WMI if Windows API failed
    if memory_info.installed_mb.is_none() {
        if let Ok(wmi_con) = wmi::WMIConnection::new() {
            if let Ok(results) = wmi_con.query::<serde_json::Value>("SELECT TotalPhysicalMemory FROM Win32_ComputerSystem") {
                if let Some(computer) = results.get(0) {
                    if let Some(total_memory) = computer.get("TotalPhysicalMemory").and_then(|v| v.as_str()) {
                        if let Ok(bytes) = total_memory.parse::<u64>() {
                            memory_info.installed_mb = Some(bytes / (1024 * 1024));
                        }
                    }
                }
            }
        }
    }
    
    memory_info
}