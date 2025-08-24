use crate::models::DiskInfo;
use windows::Win32::Storage::FileSystem;
use windows::Win32::Foundation::HANDLE;
use windows::core::{PWSTR, PCWSTR};
use std::mem::zeroed;

pub fn collect_disk_info() -> Vec<DiskInfo> {
    let mut disks = Vec::new();
    
    // Get available drive letters
    unsafe {
        let mut buffer = [0u16; 256];
        let len = FileSystem::GetLogicalDriveStringsW(buffer.len() as u32, &mut buffer);
        
        if len > 0 && len < buffer.len() as u32 {
            let mut pos = 0;
            while pos < len as usize {
                // Extract drive letter
                let drive_letter = String::from_utf16_lossy(&buffer[pos..pos+3]);
                
                // Get drive type
                let drive_type = FileSystem::GetDriveTypeW(PCWSTR::from_raw(buffer[pos..].as_ptr()));
                
                // Only process fixed drives (hard disks)
                if drive_type == FileSystem::DRIVE_FIXED {
                    let mut disk_info = DiskInfo::default();
                    disk_info.drive_letter = Some(drive_letter.trim_end_matches('\\').to_string());
                    
                    // Get filesystem type
                    let mut fs_buffer = [0u16; 32];
                    let mut volume_name_buffer = [0u16; 256];
                    let mut volume_serial_number = 0u32;
                    let mut max_component_length = 0u32;
                    let mut file_system_flags = 0u32;
                    
                    if FileSystem::GetVolumeInformationW(
                        PCWSTR::from_raw(buffer[pos..].as_ptr()),
                        &mut volume_name_buffer,
                        &mut volume_serial_number,
                        &mut max_component_length,
                        &mut file_system_flags,
                        &mut fs_buffer
                    ).is_ok() {
                        let fs_type = String::from_utf16_lossy(&fs_buffer)
                            .trim_end_matches('\0')
                            .to_string();
                        if !fs_type.is_empty() {
                            disk_info.fs_type = Some(fs_type);
                        }
                    }
                    
                    // Get free space and total size
                    let mut free_bytes_available = 0u64;
                    let mut total_bytes = 0u64;
                    let mut total_free_bytes = 0u64;
                    
                    if FileSystem::GetDiskFreeSpaceExW(
                        PCWSTR::from_raw(buffer[pos..].as_ptr()),
                        Some(&mut free_bytes_available),
                        Some(&mut total_bytes),
                        Some(&mut total_free_bytes)
                    ).is_ok() {
                        // Convert bytes to gigabytes with 2 decimal precision
                        disk_info.total_gb = Some((total_bytes as f64 / (1024.0 * 1024.0 * 1024.0)).round() / 100.0);
                        disk_info.free_gb = Some((free_bytes_available as f64 / (1024.0 * 1024.0 * 1024.0)).round() / 100.0);
                    }
                    
                    disks.push(disk_info);
                }
                
                // Move to next drive string
                while buffer[pos] != 0 {
                    pos += 1;
                }
                pos += 1; // Skip the null terminator
            }
        }
    }
    
    disks
}