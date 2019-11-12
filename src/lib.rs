use dns_lookup::{lookup_host, LookupError};
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::net::{AddrParseError, IpAddr};
use std::str::FromStr;

#[derive(Debug)]
pub enum IpaddrConversionError {
    LookupError(LookupError),
    AddrParseError(AddrParseError),
    IoError(std::io::Error),
}

impl fmt::Display for IpaddrConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpaddrConversionError::LookupError(ref err) => write!(f, "LookupError {:?}", err),
            IpaddrConversionError::AddrParseError(ref err) => write!(f, "AddrParseError {}", err),
            IpaddrConversionError::IoError(ref err) => write!(f, "IoError {}", err),
        }
    }
}
impl Error for IpaddrConversionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<LookupError> for IpaddrConversionError {
    fn from(err: LookupError) -> IpaddrConversionError {
        IpaddrConversionError::LookupError(err)
    }
}

impl From<AddrParseError> for IpaddrConversionError {
    fn from(err: AddrParseError) -> IpaddrConversionError {
        IpaddrConversionError::AddrParseError(err)
    }
}

impl From<std::io::Error> for IpaddrConversionError {
    fn from(err: std::io::Error) -> IpaddrConversionError {
        IpaddrConversionError::IoError(err)
    }
}

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
        let result = is_ipaddrv4("fred.barney.will.mork");
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
