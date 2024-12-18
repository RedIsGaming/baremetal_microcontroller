use crate::colors::toggle::ColorLed;
use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use cortex_m_semihosting::hprintln;

#[derive(Debug, Default)]
pub struct GreenLed;

impl ColorLed for GreenLed {
    fn enable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr12().set_bit());
        hprintln!("The green led is triggered.")
    }
    
    fn disable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr12().clear_bit())
    }
}

#[derive(Debug, Default)]
pub struct OrangeLed;

impl ColorLed for OrangeLed {
    fn enable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr13().set_bit());
        hprintln!("The orange led is triggered.")
    }
    
    fn disable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr13().clear_bit())
    }
}

#[derive(Debug, Default)]
pub struct RedLed;

impl ColorLed for RedLed {
    fn enable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr14().set_bit());
        hprintln!("The red led is triggered.")
    }
    
    fn disable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr14().clear_bit())
    }
}

#[derive(Debug, Default)]
pub struct BlueLed;

impl ColorLed for BlueLed {
    fn enable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr15().set_bit());
        hprintln!("The blue led is triggered.")
    }
    
    fn disable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr15().clear_bit())
    }
}
