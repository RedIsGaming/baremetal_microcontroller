#![no_std]

pub mod colors;
mod user_button;
mod state;
pub mod registers;
mod errors;

extern crate alloc;

use crate::{alloc::boxed::Box, colors::led::RedLed, errors::pin::PinError};
use core::fmt;

use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use alloc_cortex_m::CortexMHeap;
use cortex_m::delay::Delay;

#[global_allocator]
static GLOBAL_ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub trait LedState: fmt::Debug {
    fn next(self: Box<Self>, color: &Stm32Peripherals, delay: &mut Delay) -> Result<Box<dyn LedState>, PinError>;
    fn delay_status(&self, delay: &mut Delay);
}

impl Default for Box<dyn LedState> {
    fn default() -> Box<(dyn LedState + 'static)> {
        Box::new(RedLed)
    }
}
