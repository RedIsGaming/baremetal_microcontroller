use core::{convert, fmt};

#[derive(Debug)]
pub enum PinError {
    GPIOLedError(convert::Infallible),
    BridgeSensorError(&'static str),
}

impl From<convert::Infallible> for PinError {
    fn from(value: convert::Infallible) -> Self {
        PinError::GPIOLedError(value)
    }
}

impl From<&'static str> for PinError {
    fn from(value: &'static str) -> Self {
        PinError::BridgeSensorError(value)
    }
}

impl fmt::Display for PinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PinError::GPIOLedError(err) => write!(f, "{}", err),
            PinError::BridgeSensorError(err) => write!(f, "{}", err),
        }
    }
}
