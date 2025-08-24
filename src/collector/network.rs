use crate::models::NetworkInfo;
use windows::Win32::NetworkManagement::IpHelper;
use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
use std::mem::{size_of, zeroed};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub fn collect_network_info() -> Vec<NetworkInfo> {
    let mut network_interfaces = Vec::new();
    
    unsafe {
        // First call to get the required buffer size
        let mut buffer_size = 0u32;
        let result = IpHelper::GetAdaptersAddresses(
            IpHelper::AF_UNSPEC,
            IpHelper::GAA_FLAG_INCLUDE_PREFIX,
            None,
            None,
            &mut buffer_size
        );
        
        // Check if we got the expected error (buffer too small)
        if result == ERROR_BUFFER_OVERFLOW.0 {
            // Allocate buffer of the required size
            let mut buffer = vec![0u8; buffer_size as usize];
            let adapter_addresses = buffer.as_mut_ptr() as *mut IpHelper::IP_ADAPTER_ADDRESSES_LH;
            
            // Second call with properly sized buffer
            let result = IpHelper::GetAdaptersAddresses(
                IpHelper::AF_UNSPEC,
                IpHelper::GAA_FLAG_INCLUDE_PREFIX,
                None,
                Some(adapter_addresses),
                &mut buffer_size
            );
            
            if result == 0 { // NO_ERROR
                // Iterate through the linked list of adapters
                let mut current_adapter = adapter_addresses;
                
                while !current_adapter.is_null() {
                    let adapter = &*current_adapter;
                    
                    // Skip loopback and disconnected adapters
                    if adapter.IfType != IpHelper::IF_TYPE_SOFTWARE_LOOPBACK && 
                       adapter.OperStatus == IpHelper::IfOperStatusUp {
                        
                        let mut network_info = NetworkInfo::default();
                        
                        // Get interface name
                        if !adapter.FriendlyName.is_null() {
                            let name = windows::core::PWSTR::from_raw(adapter.FriendlyName);
                            network_info.interface_name = Some(name.to_string().unwrap_or_default());
                        }
                        
                        // Get MAC address
                        if adapter.PhysicalAddressLength > 0 {
                            let mac = adapter.PhysicalAddress
                                .iter()
                                .take(adapter.PhysicalAddressLength as usize)
                                .map(|b| format!("{:02X}", b))
                                .collect::<Vec<String>>()
                                .join(":");
                            
                            network_info.mac_address = Some(mac);
                        }
                        
                        // Get IP addresses
                        let mut ipv4_addresses = Vec::new();
                        let mut ipv6_addresses = Vec::new();
                        
                        let mut unicast_address = adapter.FirstUnicastAddress;
                        while !unicast_address.is_null() {
                            let address = &*unicast_address;
                            
                            if let Some(ip_addr) = socket_address_to_ip(&address.Address) {
                                match ip_addr {
                                    IpAddr::V4(ipv4) => ipv4_addresses.push(ipv4.to_string()),
                                    IpAddr::V6(ipv6) => ipv6_addresses.push(ipv6.to_string()),
                                }
                            }
                            
                            unicast_address = address.Next;
                        }
                        
                        if !ipv4_addresses.is_empty() {
                            network_info.ipv4_addresses = Some(ipv4_addresses);
                        }
                        
                        if !ipv6_addresses.is_empty() {
                            network_info.ipv6_addresses = Some(ipv6_addresses);
                        }
                        
                        // Only add interfaces with at least some information
                        if network_info.interface_name.is_some() && 
                           (network_info.ipv4_addresses.is_some() || network_info.ipv6_addresses.is_some()) {
                            network_interfaces.push(network_info);
                        }
                    }
                    
                    current_adapter = adapter.Next;
                }
            }
        }
    }
    
    network_interfaces
}

// Helper function to convert a socket address to an IP address
unsafe fn socket_address_to_ip(socket_addr: &windows::Win32::Networking::WinSock::SOCKET_ADDRESS) -> Option<IpAddr> {
    if socket_addr.lpSockaddr.is_null() {
        return None;
    }
    
    let sockaddr = &*(socket_addr.lpSockaddr as *const windows::Win32::Networking::WinSock::SOCKADDR);
    
    match sockaddr.sa_family {
        windows::Win32::Networking::WinSock::AF_INET => {
            let addr_in = &*(socket_addr.lpSockaddr as *const windows::Win32::Networking::WinSock::SOCKADDR_IN);
            let bytes = addr_in.sin_addr.S_un.S_addr.to_ne_bytes();
            Some(IpAddr::V4(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3])))
        },
        windows::Win32::Networking::WinSock::AF_INET6 => {
            let addr_in6 = &*(socket_addr.lpSockaddr as *const windows::Win32::Networking::WinSock::SOCKADDR_IN6);
            let bytes = addr_in6.sin6_addr.u.Byte;
            Some(IpAddr::V6(Ipv6Addr::new(
                ((bytes[0] as u16) << 8) | bytes[1] as u16,
                ((bytes[2] as u16) << 8) | bytes[3] as u16,
                ((bytes[4] as u16) << 8) | bytes[5] as u16,
                ((bytes[6] as u16) << 8) | bytes[7] as u16,
                ((bytes[8] as u16) << 8) | bytes[9] as u16,
                ((bytes[10] as u16) << 8) | bytes[11] as u16,
                ((bytes[12] as u16) << 8) | bytes[13] as u16,
                ((bytes[14] as u16) << 8) | bytes[15] as u16,
            )))
        },
        _ => None
    }
}