use serde_json::Value as JsonValue;

const SOURCE_IP_KEYS: &[&str] = &["sourceIp", "source_ip", "clientIp", "client_ip"];

/// Normalize a client IP for equality and indexing.
///
/// - trim
/// - strip `:port` from IPv4 (`1.2.3.4:5678`) and `[host]:port`
/// - map `::ffff:1.2.3.4` → `1.2.3.4`
/// - lowercase (IPv6 hex)
pub(crate) fn normalize_source_ip(raw: &str) -> String {
    let s = raw.trim();
    if s.is_empty() {
        return String::new();
    }

    let host = if let Some(rest) = s.strip_prefix('[') {
        match rest.find(']') {
            Some(end) => rest[..end].to_string(),
            None => s.to_string(),
        }
    } else if is_ipv4_with_port(s) {
        s.rsplit_once(':')
            .map(|(h, _)| h.to_string())
            .unwrap_or_else(|| s.to_string())
    } else {
        s.to_string()
    };

    let lower = host.to_ascii_lowercase();
    lower
        .strip_prefix("::ffff:")
        .map(str::to_string)
        .unwrap_or(lower)
}

fn is_ipv4_with_port(s: &str) -> bool {
    let Some((host, port)) = s.rsplit_once(':') else {
        return false;
    };
    if host.contains(':') || port.is_empty() || !port.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let mut parts = host.split('.');
    let mut n = 0;
    for part in parts.by_ref() {
        n += 1;
        if n > 4 || part.parse::<u8>().is_err() {
            return false;
        }
    }
    n == 4
}

pub(crate) fn extract_source_ip(metadata: Option<&JsonValue>) -> String {
    let Some(JsonValue::Object(map)) = metadata else {
        return String::new();
    };
    for key in SOURCE_IP_KEYS {
        let Some(value) = map.get(*key) else {
            continue;
        };
        let raw = match value {
            JsonValue::String(s) => s.as_str(),
            JsonValue::Null => continue,
            other => {
                if let Some(s) = other.as_str() {
                    s
                } else {
                    continue;
                }
            }
        };
        let normalized = normalize_source_ip(raw);
        if !normalized.is_empty() {
            return normalized;
        }
    }
    String::new()
}

pub(crate) fn metadata_with_source_ip(metadata: Option<JsonValue>, source_ip: &str) -> JsonValue {
    let mut map = match metadata {
        Some(JsonValue::Object(m)) => m,
        _ => serde_json::Map::new(),
    };
    map.insert(
        "sourceIp".to_string(),
        JsonValue::String(source_ip.to_string()),
    );
    JsonValue::Object(map)
}

pub(crate) fn matches_source_ip_filter(metadata: Option<&JsonValue>, filter: &str) -> bool {
    let ip = extract_source_ip(metadata);
    if filter == "__empty__" {
        return ip.is_empty();
    }
    ip == normalize_source_ip(filter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn normalize_trims_and_strips_ipv4_port() {
        assert_eq!(normalize_source_ip("  1.2.3.4:5678  "), "1.2.3.4");
        assert_eq!(normalize_source_ip("1.2.3.4"), "1.2.3.4");
    }

    #[test]
    fn normalize_maps_ipv4_mapped_and_lowercase_ipv6() {
        assert_eq!(normalize_source_ip("::ffff:127.0.0.1"), "127.0.0.1");
        assert_eq!(normalize_source_ip("::FFFF:127.0.0.1"), "127.0.0.1");
        assert_eq!(normalize_source_ip("2001:DB8::1"), "2001:db8::1");
    }

    #[test]
    fn normalize_strips_bracketed_host() {
        assert_eq!(normalize_source_ip("[::ffff:10.0.0.1]:443"), "10.0.0.1");
        assert_eq!(normalize_source_ip("[2001:DB8::1]:443"), "2001:db8::1");
    }

    #[test]
    fn extract_prefers_source_ip_keys_in_order() {
        assert_eq!(
            extract_source_ip(Some(
                &json!({"client_ip": "9.9.9.9", "sourceIp": "1.1.1.1"})
            )),
            "1.1.1.1"
        );
        assert_eq!(
            extract_source_ip(Some(&json!({"source_ip": "2.2.2.2"}))),
            "2.2.2.2"
        );
        assert_eq!(
            extract_source_ip(Some(&json!({"clientIp": "3.3.3.3"}))),
            "3.3.3.3"
        );
        assert_eq!(extract_source_ip(Some(&json!({}))), "");
        assert_eq!(extract_source_ip(None), "");
    }

    #[test]
    fn empty_filter_matches_missing_or_blank() {
        assert!(matches_source_ip_filter(Some(&json!({})), "__empty__"));
        assert!(matches_source_ip_filter(
            Some(&json!({"sourceIp": "  "})),
            "__empty__"
        ));
        assert!(!matches_source_ip_filter(
            Some(&json!({"sourceIp": "1.2.3.4"})),
            "__empty__"
        ));
        assert!(matches_source_ip_filter(
            Some(&json!({"sourceIp": "::ffff:1.2.3.4"})),
            "1.2.3.4"
        ));
    }
}
