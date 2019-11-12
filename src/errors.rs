use dns_lookup::LookupError;
use std::error::Error;
use std::fmt;
use std::net::AddrParseError;

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
