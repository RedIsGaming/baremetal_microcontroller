#![no_std]

pub mod color_led;
mod led;
mod user_button;

extern crate alloc;

use crate::{alloc::boxed::Box, color_led::{GreenLed, OrangeLed, RedLed}, led::Leds, user_button::UserButton};
use core::{convert::Infallible, fmt};
use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use alloc_cortex_m::CortexMHeap;
use cortex_m::delay::Delay;

#[global_allocator]
static GLOBAL_ALLOCATOR: CortexMHeap = CortexMHeap::empty();

pub trait LedState: fmt::Debug {
    fn next(self: Box<Self>, color: &Stm32Peripherals, delay: &mut Delay) -> Result<Box<dyn LedState>, Infallible>;
    fn delay_status(&self, delay: &mut Delay) -> Leds;
}

impl Default for Box<dyn LedState> {
    fn default() -> Box<(dyn LedState + 'static)> {
        Box::new(RedLed)
    }
}

impl LedState for GreenLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals, _delay: &mut Delay) -> Result<Box<dyn LedState>, Infallible> {
        GreenLed::disable_led(GreenLed, color);
        OrangeLed::enable_led(OrangeLed, color);
        Ok(Box::new(OrangeLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) -> Leds {
        let leds = Leds::new(false, 3000);
        Leds::delay(leds, delay)
    }
}

impl LedState for OrangeLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals, delay: &mut Delay) -> Result<Box<dyn LedState>, Infallible> {
        OrangeLed::disable_led(OrangeLed, color);
        RedLed::enable_led(RedLed, color);

        if color.GPIOA.idr.read().idr0().bit_is_set() {
            UserButton::bridge_state(&UserButton::Open);
            Leds::delay(Leds::new(false, 2000), delay);
            RedLed::disable_led(RedLed, color);

            return Ok(Box::new(OrangeLed));
        }

        Ok(Box::new(RedLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) -> Leds {
        let leds = Leds::new(false, 1000);
        Leds::delay(leds, delay)
    }
}

impl LedState for RedLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals, _delay: &mut Delay) -> Result<Box<dyn LedState>, Infallible> {
        RedLed::disable_led(RedLed, color);
        GreenLed::enable_led(GreenLed, color);
        Ok(Box::new(GreenLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) -> Leds {
        let leds = Leds::new(false, 4000);
        Leds::delay(leds, delay)
    }
}
