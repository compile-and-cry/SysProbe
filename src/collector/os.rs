use crate::models::OsInfo;
use windows::Win32::System::SystemInformation;
use windows::core::PWSTR;
use std::mem::zeroed;

pub fn collect_os_info() -> OsInfo {
    let mut os_info = OsInfo::default();
    os_info.family = "Windows".to_string();
    
    // Get Windows version information
    unsafe {
        let mut os_version_info: SystemInformation::OSVERSIONINFOW = zeroed();
        os_version_info.dwOSVersionInfoSize = std::mem::size_of::<SystemInformation::OSVERSIONINFOW>() as u32;
        
        if SystemInformation::GetVersionExW(&mut os_version_info as *mut _ as *mut _).is_ok() {
            os_info.build = Some(os_version_info.dwBuildNumber.to_string());
        }
    }
    
    // Get Windows edition and version from registry
    if let Ok(hklm) = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE) {
        if let Ok(current_version) = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion") {
            os_info.edition = current_version.get_value::<String, _>("EditionID").ok();
            os_info.version = current_version.get_value::<String, _>("DisplayVersion").ok();
            os_info.product_id = current_version.get_value::<String, _>("ProductId")
                .ok()
                .map(|id| {
                    // Only include partial product ID for privacy
                    if id.len() > 8 {
                        format!("{}...", &id[0..5])
                    } else {
                        id
                    }
                });
        }
    }
    
    // Get system architecture
    unsafe {
        let mut system_info: SystemInformation::SYSTEM_INFO = zeroed();
        SystemInformation::GetNativeSystemInfo(&mut system_info);
        
        os_info.arch = Some(match system_info.Anonymous.Anonymous.wProcessorArchitecture {
            SystemInformation::PROCESSOR_ARCHITECTURE_AMD64 => "x86_64".to_string(),
            SystemInformation::PROCESSOR_ARCHITECTURE_ARM64 => "arm64".to_string(),
            SystemInformation::PROCESSOR_ARCHITECTURE_INTEL => "x86".to_string(),
            SystemInformation::PROCESSOR_ARCHITECTURE_ARM => "arm".to_string(),
            _ => "unknown".to_string(),
        });
    }
    
    os_info
}