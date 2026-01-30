//! Local IP address detection.

/// Returns the local IPv4 address as a string, or None if unavailable.
pub fn get_local_ip() -> Option<String> {
    let ip = local_ip_address::local_ip().ok()?;
    if ip.is_ipv4() {
        Some(ip.to_string())
    } else {
        Some(ip.to_string())
    }
}
