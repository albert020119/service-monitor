fn strip_scheme_and_path(raw: &str) -> String {
    let mut s = raw.to_string();

    if let Some(pos) = s.find("://") {
        s = s.split_at(pos + 3).1.to_string();
    }

    if let Some(pos) = s.find('/') {
        s = s[..pos].to_string();
    }

    s
}

/// Best-effort parsing of a "host[:port]" from a URL-like string.
///
/// - Strips `scheme://` if present
/// - Strips any path after `/`
/// - If a numeric `:port` suffix is present, uses it; otherwise uses `default_port`
pub fn normalize_host_port(raw: &str, default_port: u16) -> (String, u16) {
    let s = strip_scheme_and_path(raw);

    // Handle bracketed IPv6: [::1]:443
    if s.starts_with('[') {
        if let Some(end) = s.find(']') {
            let host = s[1..end].to_string();
            let rest = &s[end + 1..];
            if let Some(port_str) = rest.strip_prefix(':') {
                if let Ok(p) = port_str.parse::<u16>() {
                    return (host, p);
                }
            }
            return (host, default_port);
        }
    }

    // Fallback: split on last ':' and treat it as a port if numeric.
    if let Some(pos) = s.rfind(':') {
        if let Ok(p) = s[pos + 1..].parse::<u16>() {
            let host = s[..pos].to_string();
            return (host, p);
        }
    }

    (s, default_port)
}

/// Best-effort extraction of a hostname from a URL-like string.
///
/// - Strips `scheme://` if present
/// - Strips any path after `/`
/// - Strips a numeric `:port` suffix if present
pub fn normalize_host(raw: &str) -> String {
    let s = strip_scheme_and_path(raw);

    // Handle bracketed IPv6: [::1]:443 -> ::1
    if s.starts_with('[') {
        if let Some(end) = s.find(']') {
            return s[1..end].to_string();
        }
    }

    if let Some(pos) = s.rfind(':') {
        if s[pos + 1..].parse::<u16>().is_ok() {
            return s[..pos].to_string();
        }
    }

    s
}
