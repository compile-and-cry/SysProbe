use crate::models::TallyInfo;
use std::time::Duration;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

pub fn collect_tally_info(timeout: Duration, http_endpoint: Option<String>) -> TallyInfo {
    let mut tally_info = TallyInfo {
        installed: false,
        variant: None,
        version: None,
        edition: None,
        install_path: None,
        detection_source: None,
    };
    
    // Check registry uninstall keys
    if let Some((variant, version, path)) = check_registry_uninstall() {
        tally_info.installed = true;
        tally_info.variant = Some(variant);
        tally_info.version = Some(version);
        tally_info.install_path = Some(path);
        tally_info.detection_source = Some("registry_uninstall".to_string());
    }
    
    // Check vendor registry keys
    else if let Some((variant, version, path)) = check_vendor_registry() {
        tally_info.installed = true;
        tally_info.variant = Some(variant);
        tally_info.version = Some(version);
        tally_info.install_path = Some(path);
        tally_info.detection_source = Some("registry_vendor".to_string());
    }
    
    // Check default install paths
    else if let Some((variant, version, path)) = check_default_install_paths() {
        tally_info.installed = true;
        tally_info.variant = Some(variant);
        tally_info.version = Some(version);
        tally_info.install_path = Some(path);
        tally_info.detection_source = Some("file_version".to_string());
    }
    
    // Optional: Check HTTP endpoint if Tally is installed
    if tally_info.installed && http_endpoint.is_some() {
        #[cfg(feature = "tally-xml")]
        if let Some(http_info) = check_tally_http(http_endpoint.unwrap(), timeout) {
            // Update with potentially more accurate information from HTTP
            if let Some(variant) = http_info.0 {
                tally_info.variant = Some(variant);
            }
            if let Some(version) = http_info.1 {
                tally_info.version = Some(version);
            }
            if let Some(edition) = http_info.2 {
                tally_info.edition = Some(edition);
            }
            tally_info.detection_source = Some("http".to_string());
        }
    }
    
    // Optional: Check ODBC if Tally is installed
    #[cfg(feature = "tally-odbc")]
    if tally_info.installed {
        if let Some(odbc_info) = check_tally_odbc(timeout) {
            // Update with potentially more accurate information from ODBC
            if let Some(variant) = odbc_info.0 {
                tally_info.variant = Some(variant);
            }
            if let Some(version) = odbc_info.1 {
                tally_info.version = Some(version);
            }
            tally_info.detection_source = Some("odbc".to_string());
        }
    }
    
    tally_info
}

// Check Windows registry uninstall keys for Tally
fn check_registry_uninstall() -> Option<(String, String, String)> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // Try both 32-bit and 64-bit registry views
    let uninstall_keys = [
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall"
    ];
    
    for uninstall_key in uninstall_keys.iter() {
        if let Ok(uninstall) = hklm.open_subkey(uninstall_key) {
            // Enumerate all subkeys
            for subkey_result in uninstall.enum_keys() {
                if let Ok(subkey_name) = subkey_result {
                    if let Ok(subkey) = uninstall.open_subkey(&subkey_name) {
                        // Check if DisplayName contains "Tally"
                        if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                            if display_name.contains("Tally") {
                                // Extract variant (ERP9 or Prime)
                                let variant = if display_name.contains("Prime") {
                                    "TallyPrime".to_string()
                                } else if display_name.contains("ERP") || display_name.contains("ERP9") {
                                    "TallyERP9".to_string()
                                } else {
                                    "Tally".to_string()
                                };
                                
                                // Get version
                                let version = subkey.get_value::<String, _>("DisplayVersion")
                                    .unwrap_or_else(|_| "Unknown".to_string());
                                
                                // Get install location
                                let install_path = subkey.get_value::<String, _>("InstallLocation")
                                    .unwrap_or_else(|_| "Unknown".to_string());
                                
                                return Some((variant, version, install_path));
                            }
                        }
                    }
                }
            }
        }
    }
    
    None
}

// Check Tally vendor registry keys
fn check_vendor_registry() -> Option<(String, String, String)> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // Try both 32-bit and 64-bit registry views
    let vendor_keys = [
        "SOFTWARE\\Tally Solutions",
        "SOFTWARE\\WOW6432Node\\Tally Solutions"
    ];
    
    for vendor_key in vendor_keys.iter() {
        if let Ok(vendor) = hklm.open_subkey(vendor_key) {
            // Look for product subkeys
            for product_result in vendor.enum_keys() {
                if let Ok(product_name) = product_result {
                    if let Ok(product) = vendor.open_subkey(&product_name) {
                        // Determine variant
                        let variant = if product_name.contains("Prime") {
                            "TallyPrime".to_string()
                        } else if product_name.contains("ERP") || product_name.contains("ERP9") {
                            "TallyERP9".to_string()
                        } else {
                            "Tally".to_string()
                        };
                        
                        // Try to get version and install path
                        let version = product.get_value::<String, _>("Version")
                            .unwrap_or_else(|_| "Unknown".to_string());
                        
                        let install_path = product.get_value::<String, _>("InstallDir")
                            .unwrap_or_else(|_| "Unknown".to_string());
                        
                        return Some((variant, version, install_path));
                    }
                }
            }
        }
    }
    
    None
}

// Check default install paths for Tally
fn check_default_install_paths() -> Option<(String, String, String)> {
    let default_paths = [
        "C:\\Program Files\\Tally\\ERP9",
        "C:\\Program Files\\Tally\\TallyPrime",
        "C:\\Program Files\\Tally\\TallyERP9",
        "C:\\Program Files (x86)\\Tally\\ERP9",
        "C:\\Program Files (x86)\\Tally\\TallyPrime",
        "C:\\Program Files (x86)\\Tally\\TallyERP9",
    ];
    
    for path_str in default_paths.iter() {
        let path = Path::new(path_str);
        if path.exists() {
            let tally_exe = path.join("Tally.exe");
            
            if tally_exe.exists() {
                // Determine variant from path
                let variant = if path_str.contains("Prime") {
                    "TallyPrime".to_string()
                } else if path_str.contains("ERP9") || path_str.contains("ERP") {
                    "TallyERP9".to_string()
                } else {
                    "Tally".to_string()
                };
                
                // Try to get file version info
                // This is a simplified version - in a real implementation, you would use
                // Windows API to get file version info
                let version = "Unknown".to_string();
                
                return Some((variant, version, path_str.to_string()));
            }
        }
    }
    
    None
}

// Optional: Check Tally HTTP endpoint
#[cfg(feature = "tally-xml")]
fn check_tally_http(endpoint: String, timeout: Duration) -> Option<(Option<String>, Option<String>, Option<String>)> {
    use std::net::TcpStream;
    use std::io::{Read, Write};
    use quick_xml::de::from_str;
    use serde::Deserialize;
    
    #[derive(Debug, Deserialize)]
    struct TallyResponse {
        #[serde(rename = "PRODUCT")]
        product: Option<String>,
        
        #[serde(rename = "VERSION")]
        version: Option<String>,
        
        #[serde(rename = "EDITION")]
        edition: Option<String>,
    }
    
    // Set a shorter timeout for the HTTP request
    let http_timeout = std::cmp::min(timeout, Duration::from_millis(200));
    
    // Try to connect to the Tally HTTP endpoint
    if let Ok(stream) = TcpStream::connect_timeout(&endpoint.parse().ok()?, http_timeout) {
        stream.set_read_timeout(Some(http_timeout)).ok()?;
        stream.set_write_timeout(Some(http_timeout)).ok()?;
        
        let mut stream = stream;
        
        // Simple XML request to get Tally product info
        let request = "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>PRODUCT</TALLYREQUEST></HEADER></ENVELOPE>";
        
        // Send request
        if stream.write_all(request.as_bytes()).is_err() {
            return None;
        }
        
        // Read response
        let mut response = String::new();
        if stream.read_to_string(&mut response).is_err() {
            return None;
        }
        
        // Parse XML response
        if let Ok(tally_response) = from_str::<TallyResponse>(&response) {
            let variant = tally_response.product.map(|p| {
                if p.contains("Prime") {
                    "TallyPrime".to_string()
                } else if p.contains("ERP") || p.contains("ERP9") {
                    "TallyERP9".to_string()
                } else {
                    p
                }
            });
            
            return Some((variant, tally_response.version, tally_response.edition));
        }
    }
    
    None
}

// Optional: Check Tally ODBC
#[cfg(feature = "tally-odbc")]
fn check_tally_odbc(timeout: Duration) -> Option<(Option<String>, Option<String>)> {
    use odbc_api::{Environment, ConnectionOptions};
    
    // Create ODBC environment
    let env = Environment::new()?;
    
    // Check if TallyODBC DSN exists
    let dsn_name = "TallyODBC";
    
    // Try to connect with timeout
    let connection = env.connect_with_connection_string(
        &format!("DSN={};TIMEOUT={}", dsn_name, timeout.as_millis()),
        ConnectionOptions::default()
    ).ok()?;
    
    // Execute a simple query to get Tally version
    let mut stmt = connection.execute("SELECT @@version AS Version, 'TallyODBC' AS Product", ()).ok()?;
    
    // Fetch the result
    if let Some(mut cursor) = stmt.fetch().ok()? {
        let mut version = String::new();
        let mut product = String::new();
        
        // Get version
        cursor.get_data(1, &mut version).ok()?;
        
        // Get product name
        cursor.get_data(2, &mut product).ok()?;
        
        // Determine variant
        let variant = if version.contains("Prime") {
            "TallyPrime".to_string()
        } else if version.contains("ERP") || version.contains("ERP9") {
            "TallyERP9".to_string()
        } else {
            product
        };
        
        return Some((Some(variant), Some(version)));
    }
    
    None
}