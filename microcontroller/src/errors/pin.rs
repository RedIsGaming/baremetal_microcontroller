use core::{convert, fmt};

#[derive(Debug)]
pub enum PinError {
    GPIOLedError(convert::Infallible),
    BridgeSensorError(&'static str),
}

impl From<convert::Infallible> for PinError {
    fn from(error: convert::Infallible) -> Self {
        PinError::GPIOLedError(error)
    }
}

impl From<&'static str> for PinError {
    fn from(error: &'static str) -> Self {
        PinError::BridgeSensorError(error)
    }
}

impl fmt::Display for PinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PinError::GPIOLedError(infallible) => write!(f, "{}", infallible),
            PinError::BridgeSensorError(err) => write!(f, "{}", err),
        }
    }
}
