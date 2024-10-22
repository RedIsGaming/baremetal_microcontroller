use stm32f4::stm32f411::Peripherals as Stm32Peripherals;

pub fn gpiod_register(stm32_peripherals: &Stm32Peripherals) {
    //GPIOD ophalen en GPIOA aanzetten
    stm32_peripherals.RCC.ahb1enr.modify(|_, w| w
        .gpioden().set_bit()
        .gpioaen().set_bit()
    );

    //pd12, 13, 14 en 15 is pin type
    stm32_peripherals.GPIOD.moder.modify(|_, w| w
        .moder12().output()
        .moder13().output()
        .moder14().output()
        .moder15().output()
    );

    stm32_peripherals.GPIOA.moder.modify(|_, w| w.moder0().input());
}
