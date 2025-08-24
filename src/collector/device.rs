use crate::models::DeviceInfo;
use windows::Win32::System::SystemInformation;
use windows::core::PWSTR;
use std::mem::zeroed;

pub fn collect_device_info() -> DeviceInfo {
    let mut device_info = DeviceInfo::default();
    
    // Get hostname
    unsafe {
        let mut buffer = [0u16; 256];
        let mut size = buffer.len() as u32;
        
        if SystemInformation::GetComputerNameExW(
            SystemInformation::COMPUTER_NAME_FORMAT_COMPUTER_NAME_DNS_HOSTNAME,
            PWSTR::from_raw(buffer.as_mut_ptr()),
            &mut size
        ).is_ok() {
            device_info.hostname = Some(
                String::from_utf16_lossy(&buffer[0..size as usize])
            );
        }
    }
    
    // Get manufacturer, model and UUID from WMI
    if let Ok(wmi_con) = wmi::WMIConnection::new() {
        // Query Win32_ComputerSystem for manufacturer and model
        if let Ok(results) = wmi_con.query::<serde_json::Value>("SELECT Manufacturer, Model FROM Win32_ComputerSystem") {
            if let Some(computer) = results.get(0) {
                if let Some(manufacturer) = computer.get("Manufacturer").and_then(|v| v.as_str()) {
                    device_info.manufacturer = Some(manufacturer.to_string());
                }
                
                if let Some(model) = computer.get("Model").and_then(|v| v.as_str()) {
                    device_info.model = Some(model.to_string());
                }
            }
        }
        
        // Query Win32_ComputerSystemProduct for UUID
        if let Ok(results) = wmi_con.query::<serde_json::Value>("SELECT UUID FROM Win32_ComputerSystemProduct") {
            if let Some(product) = results.get(0) {
                if let Some(uuid) = product.get("UUID").and_then(|v| v.as_str()) {
                    device_info.uuid = Some(uuid.to_string());
                }
            }
        }
    }
    
    device_info
}