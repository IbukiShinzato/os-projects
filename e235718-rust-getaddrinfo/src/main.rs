use addr::Addr;
use libc::{getifaddrs, ifaddrs, sockaddr, AF_INET, AF_INET6};
use std::collections::HashMap;
use std::ffi::CStr;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::ptr;

mod addr;

fn main() {
    // hash of address structure
    let mut hash: HashMap<String, Addr> = HashMap::new();

    unsafe {
        let mut ifap: *mut ifaddrs = ptr::null_mut();

        if getifaddrs(&mut ifap) != 0 {
            eprintln!("Failed to get network interfaces.");
            return;
        }

        let mut cursor = ifap;

        while !cursor.is_null() {
            let iface = &*cursor;

            if !iface.ifa_addr.is_null() {
                let sockaddr = &*iface.ifa_addr;

                let name = CStr::from_ptr(iface.ifa_name).to_string_lossy().to_string();

                let entry = hash
                    .entry(name.clone())
                    .or_insert_with(|| Addr::new(name.clone()));

                match sockaddr.sa_family as i32 {
                    AF_INET => {
                        let addr = sockaddr as *const sockaddr as *const libc::sockaddr_in;
                        let ip = (*addr).sin_addr.s_addr.to_be();
                        let ipv4 = Ipv4Addr::from(ip).to_string();
                        entry.ipv4 = ipv4;
                    }
                    AF_INET6 => {
                        let addr = sockaddr as *const sockaddr as *const libc::sockaddr_in6;
                        let ip = (*addr).sin6_addr.s6_addr;
                        let ipv6 = Ipv6Addr::from(ip).to_string();
                        entry.ipv6 = ipv6;
                    }
                    _ => {}
                }
            }

            cursor = iface.ifa_next;
        }

        libc::freeifaddrs(ifap);
    }

    // print get address
    for addr in hash.values() {
        let mut flag = false;
        let mut res = addr.name.clone() + "\n";
        if !addr.ipv4.is_empty() {
            res += "inet ";
            res += &addr.ipv4;
            res += "\n";
            flag = true;
        }
        if !addr.ipv6.is_empty() {
            res += "inet6 ";
            res += &addr.ipv6;
            res += "\n";
            flag = true;
        }

        if flag {
            println!("{}", res);
        }
    }
}
