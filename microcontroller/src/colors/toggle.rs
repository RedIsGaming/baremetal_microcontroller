use stm32f4::stm32f411::Peripherals as Stm32Peripherals;

pub trait ColorLed {
    fn enable_led(self, stm32_peripherals: &Stm32Peripherals);
    fn disable_led(self, stm32_peripherals: &Stm32Peripherals);
}
