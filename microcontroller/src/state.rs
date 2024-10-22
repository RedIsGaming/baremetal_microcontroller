use crate::{led::{GreenLed, OrangeLed, RedLed}, user_button::{BridgeState, UserButton}, LedState};
use alloc::boxed::Box;
use core::convert::Infallible;

use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use cortex_m::delay::Delay;

impl LedState for GreenLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals) -> Result<Box<dyn LedState>, Infallible> {
        GreenLed::disable_led(GreenLed, color);
        OrangeLed::enable_led(OrangeLed, color);
        Ok(Box::new(OrangeLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(3000_u32)
    }
}

impl LedState for OrangeLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals) -> Result<Box<dyn LedState>, Infallible> {
        OrangeLed::disable_led(OrangeLed, color);
        RedLed::enable_led(RedLed, color);

        if color.GPIOA.idr.read().idr0().bit_is_set() {
            return Ok(Box::new(UserButton));
        }

        Ok(Box::new(RedLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(1000_u32)
    }
}

impl LedState for RedLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals) -> Result<Box<dyn LedState>, Infallible> {
        RedLed::disable_led(RedLed, color);
        GreenLed::enable_led(GreenLed, color);
        Ok(Box::new(GreenLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(4000_u32)
    }
}

impl LedState for UserButton {
    fn next(self: Box<Self>, color: &Stm32Peripherals) -> Result<Box<dyn LedState>, Infallible> {
        UserButton::bridge_state(BridgeState::Open);
        RedLed::disable_led(RedLed, color);
        Ok(Box::new(OrangeLed))
    }

    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(2000_u32);
    }
}
