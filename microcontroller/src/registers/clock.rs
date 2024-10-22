use stm32f4::stm32f411::Peripherals as Stm32Peripherals;

pub fn clock_register(stm32_peripherals: &Stm32Peripherals) {
    //Klok instellen
    stm32_peripherals.RCC.pllcfgr.modify(|_, w| 
        unsafe {
            w.pllsrc().hse()
                .pllp().bits(0x2)
                .plln().bits(0x30)
                .pllm().bits(0x4)
        }
    );

    stm32_peripherals.RCC.cr.modify(|_, w| w.pllon().on());
    stm32_peripherals.RCC.cfgr.modify(|_, w| w.sw().pll().ppre1().div2());
    stm32_peripherals.RCC.cr.modify(|_, w| w.hsion().off());
}
