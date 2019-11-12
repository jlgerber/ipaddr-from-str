use dns_lookup::lookup_host;
use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;
use std::str::FromStr;

pub mod errors;
pub use errors::IpaddrConversionError;

/// Test if input is an ipaddr v4
///
pub fn is_ipaddrv4(input: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$").unwrap();
    }
    if let Some(val) = RE.captures_iter(input).next() {
        val.iter() // we only have one group
            .skip(1) // drop the whole match
            .map(|v| v.map_or(None, |x| x.as_str().parse::<u8>().ok()))
            .all(|v| v.is_some())
    } else {
        false
    }
}

/// Retrieve an address...
pub fn get_ipaddr(hostname: &str) -> Result<Vec<IpAddr>, IpaddrConversionError> {
    if is_ipaddrv4(hostname) {
        Ok(vec![IpAddr::from_str(hostname)?])
    } else {
        Ok(lookup_host(hostname)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    //
    #[test]
    fn can_identify_min_ipv4_addrs() {
        let result = is_ipaddrv4("0.0.0.0");
        assert_eq!(result, true);
    }
    #[test]
    fn can_identify_max_ipv4_addrs() {
        let result = is_ipaddrv4("255.255.255.255");
        assert_eq!(result, true);
    }
    #[test]
    fn can_identify_bad_ipv4_addrs() {
        let result = is_ipaddrv4("fred.barney.will");
        assert_eq!(result, false);
    }
    #[test]
    fn can_identify_out_of_bound_ipv4_addrs() {
        let result = is_ipaddrv4("256.0.0.0");
        assert_eq!(result, false);
    }
    #[test]
    fn can_get_ipaddr_from_ip() {
        let ip = get_ipaddr("0.0.0.0").unwrap();
        let expect = vec![IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))];
        assert_eq!(expect, ip);
    }
}
