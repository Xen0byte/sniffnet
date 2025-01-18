use crate::networking::types::ip_collection::AddressCollection;
use once_cell::sync::Lazy;
use std::net::IpAddr;
use std::str::FromStr;

pub struct Bogon {
    pub range: AddressCollection,
    pub description: &'static str,
}

// IPv4 bogons

static THIS_NETWORK: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("0.0.0.0-0.255.255.255").unwrap(),
    description: "\"this\" network",
});

static PRIVATE_USE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new(
        "10.0.0.0-10.255.255.255, 172.16.0.0-172.31.255.255, 192.168.0.0-192.168.255.255",
    )
    .unwrap(),
    description: "private-use networks",
});

static CARRIER_GRADE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("100.64.0.0-100.127.255.255").unwrap(),
    description: "carrier-grade NAT",
});

static LOOPBACK: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("127.0.0.0-127.255.255.255").unwrap(),
    description: "loopback",
});

static LINK_LOCAL: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("169.254.0.0-169.254.255.255").unwrap(),
    description: "link local",
});

static IETF_PROTOCOL: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("192.0.0.0-192.0.0.255").unwrap(),
    description: "IETF protocol assignments",
});

static TEST_NET_1: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("192.0.2.0-192.0.2.255").unwrap(),
    description: "TEST-NET-1",
});

static NETWORK_INTERCONNECT: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("198.18.0.0-198.19.255.255").unwrap(),
    description: "network interconnect device benchmark testing",
});

static TEST_NET_2: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("198.51.100.0-198.51.100.255").unwrap(),
    description: "TEST-NET-2",
});

static TEST_NET_3: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("203.0.113.0-203.0.113.255").unwrap(),
    description: "TEST-NET-3",
});

static FUTURE_USE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("240.0.0.0-255.255.255.255").unwrap(),
    description: "reserved for future use",
});

// IPv6 bogons

static NODE_SCOPE_UNSPECIFIED: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("::").unwrap(),
    description: "node-scope unicast unspecified",
});

static NODE_SCOPE_LOOPBACK: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("::1").unwrap(),
    description: "node-scope unicast loopback",
});

static IPV4_MAPPED: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("::ffff:0.0.0.0-::ffff:255.255.255.255").unwrap(),
    description: "IPv4-mapped",
});

static IPV4_COMPATIBLE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("::-::255.255.255.255").unwrap(),
    description: "IPv4-compatible",
});

static REMOTELY_TRIGGERED: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("100::-100::ffff:ffff:ffff:ffff").unwrap(),
    description: "remotely triggered black hole",
});

static ORCHID: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("2001:10::-2001:1f:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "ORCHID",
});

static DOCUMENTATION_PREFIX: Lazy<Bogon> = Lazy::new(|| {
    Bogon {
    range: AddressCollection::new("2001:db8::-2001:db8:ffff:ffff:ffff:ffff:ffff:ffff, 3fff::-3fff:fff:ffff:ffff:ffff:ffff:ffff:ffff")
        .unwrap(),
    description: "documentation prefix",
}
});

static ULA: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("fc00::-fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "ULA",
});

static LINK_LOCAL_UNICAST: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("fe80::-febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "link-local unicast",
});

static SITE_LOCAL_UNICAST: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: AddressCollection::new("fec0::-feff:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "site-local unicast",
});

// all bogons

static BOGONS: Lazy<Vec<&'static Bogon>> = Lazy::new(|| {
    vec![
        &THIS_NETWORK,
        &PRIVATE_USE,
        &CARRIER_GRADE,
        &LOOPBACK,
        &LINK_LOCAL,
        &IETF_PROTOCOL,
        &TEST_NET_1,
        &NETWORK_INTERCONNECT,
        &TEST_NET_2,
        &TEST_NET_3,
        &FUTURE_USE,
        &NODE_SCOPE_UNSPECIFIED,
        &NODE_SCOPE_LOOPBACK,
        &IPV4_MAPPED,
        &IPV4_COMPATIBLE,
        &REMOTELY_TRIGGERED,
        &ORCHID,
        &DOCUMENTATION_PREFIX,
        &ULA,
        &LINK_LOCAL_UNICAST,
        &SITE_LOCAL_UNICAST,
    ]
});

pub fn is_bogon(address: &str) -> Option<&'static str> {
    let ip = IpAddr::from_str(address).ok()?;
    for bogon in BOGONS.iter() {
        if bogon.range.contains(&ip) {
            return Some(bogon.description);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bogon_no() {
        assert_eq!(is_bogon("8.8.8.8"), None);
    }

    #[test]
    fn test_is_bogon_this_network() {
        assert_eq!(is_bogon("0.1.2.3"), Some("\"this\" network"));
    }
}
