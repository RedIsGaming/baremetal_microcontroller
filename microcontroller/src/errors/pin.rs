use crate::LedState;
use core::{convert, error, fmt};
use alloc::boxed::Box;

#[derive(Debug)]
pub struct Pin {
    pub source: &'static str,
    pub state: Box<dyn LedState>
}

impl Pin {
    pub fn new(source: &'static str, state: Box<dyn LedState>) -> Pin {
        Pin { 
            source, 
            state,
        }
    }
}

#[derive(Debug)]
pub enum PinError {
    GPIOLedError(convert::Infallible),
    BridgeSensorError(Pin),
}

impl From<convert::Infallible> for PinError {
    fn from(error: convert::Infallible) -> Self {
        PinError::GPIOLedError(error)
    }
}

impl From<Pin> for PinError {
    fn from(error: Pin) -> Self {
        PinError::BridgeSensorError(error)
    }
}

impl fmt::Display for Pin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An unexpected error has occurred!")
    }
}

impl error::Error for Pin {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self)
    }
}
