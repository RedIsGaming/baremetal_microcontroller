use crate::{colors::{led::*, toggle::ColorLed}, errors::pin::PinError, user_button::{BridgeState, UserButton}, LedState};
use alloc::boxed::Box;

use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use cortex_m::delay::Delay;

impl LedState for GreenLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals, _delay: &mut Delay) -> Result<Box<dyn LedState>, PinError> {
        GreenLed::disable_led(GreenLed, color);
        OrangeLed::enable_led(OrangeLed, color);
        Ok(Box::new(OrangeLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(3000_u32)
    }
}

impl LedState for OrangeLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals, _delay: &mut Delay) -> Result<Box<dyn LedState>, PinError> {
        OrangeLed::disable_led(OrangeLed, color);
        RedLed::enable_led(RedLed, color);

        if color.GPIOA.idr.read().idr0().bit_is_set() {
            //return Ok(Box::new(UserButton));
            return Err(PinError::from("The bridge sensor is broken!"))?;
        }

        Ok(Box::new(RedLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(1000_u32)
    }
}

impl LedState for RedLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals, _delay: &mut Delay) -> Result<Box<dyn LedState>, PinError> {
        RedLed::disable_led(RedLed, color);
        GreenLed::enable_led(GreenLed, color);
        Ok(Box::new(GreenLed))
    }
    
    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(4000_u32)
    }
}

impl LedState for UserButton {
    fn next(self: Box<Self>, color: &Stm32Peripherals, _delay: &mut Delay) -> Result<Box<dyn LedState>, PinError> {
        UserButton::bridge_state(BridgeState::Open);
        RedLed::disable_led(RedLed, color);
        Ok(Box::new(OrangeLed))
    }

    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(2000_u32)
    }
}

impl LedState for BlueLed {
    fn next(self: Box<Self>, color: &Stm32Peripherals, delay: &mut Delay) -> Result<Box<dyn LedState>, PinError> {
        GreenLed::disable_led(GreenLed, color);
        OrangeLed::disable_led(OrangeLed, color);
        RedLed::disable_led(RedLed, color);
        BlueLed::disable_led(BlueLed, color);
        
        delay.delay_ms(500_u32);
        BlueLed::enable_led(BlueLed, color);
        Ok(Box::new(BlueLed))
    }

    fn delay_status(&self, delay: &mut Delay) {
        delay.delay_ms(500_u32)
    }
}
