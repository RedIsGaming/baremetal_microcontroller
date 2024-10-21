use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use cortex_m_semihosting::hprintln;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GreenLed;

impl GreenLed {
    pub fn enable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr12().set_bit());
        hprintln!("The green led is triggered.")
    }
    
    pub fn disable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr12().clear_bit())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct OrangeLed;

impl OrangeLed {
    pub fn enable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr13().set_bit());
        hprintln!("The orange led is triggered.")
    }
    
    pub fn disable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr13().clear_bit())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RedLed;

impl RedLed {
    pub fn enable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr14().set_bit());
        hprintln!("The red led is triggered.")
    }
    
    pub fn disable_led(self, stm32_peripherals: &Stm32Peripherals) {
        stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr14().clear_bit())
    }
}
