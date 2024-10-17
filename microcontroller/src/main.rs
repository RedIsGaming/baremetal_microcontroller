#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::ops::{Mul, Sub};
use cortex_m_rt::entry;
// Halt on panic
use panic_semihosting as _; // panic handler
use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use cortex_m::{delay::Delay, Peripherals as CortexPeripherals};
use cortex_m_semihosting::hprintln;

fn gpiod_register(stm32_peripherals: &Stm32Peripherals) {
    //GPIOD ophalen
    stm32_peripherals.RCC.ahb1enr.modify(|_, w| w.gpioden().set_bit());

    //pd12, 13, 14 en 15 is pin type
    stm32_peripherals.GPIOD.moder.modify(|_, w| w
        .moder12().output()
        .moder13().output()
        .moder14().output()
        .moder15().output()
    )
}

fn enable_led(stm32_peripherals: &Stm32Peripherals) {
    stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr12().set_bit())
}

fn disable_led(stm32_peripherals: &Stm32Peripherals) {
    stm32_peripherals.GPIOD.odr.modify(|_, w| w.odr12().clear_bit())
}

fn clock_register(stm32_peripherals: &Stm32Peripherals) {
    //Klok instellen
    stm32_peripherals.RCC.pllcfgr.modify(|_, w| 
        unsafe {
            w.pllsrc().hse().pllp().bits(0x2).plln().bits(0x30).pllm().bits(0x4)
        }
    );

    stm32_peripherals.RCC.cr.modify(|_, w| w.pllon().on());
    stm32_peripherals.RCC.cfgr.modify(|_, w| w.sw().pll().ppre1().div2());
    stm32_peripherals.RCC.cr.modify(|_, w| w.hsion().off());
}

#[entry]
unsafe fn main() -> ! {
    if let Some(stm32_peripherals) = Stm32Peripherals::take() {
        let cortex_peripherals = CortexPeripherals::take().unwrap();

        // Create a delay abstraction based on SysTick
        let mut delay = Delay::new(cortex_peripherals.SYST, 0x7A1200_u32.mul(2).sub(1));
        let mut status = false;

        clock_register(&stm32_peripherals);
        gpiod_register(&stm32_peripherals);

        loop {
            // On for 1s, off for 1s.
            enable_led(&stm32_peripherals);
            delay.delay_ms(1000_u32);
            status ^= true;

            //dit verschijnt in een van de open terminals in vscode
            hprintln!("Led {:?}", if status { "uit!" } else { "aan!" });

            disable_led(&stm32_peripherals);
            delay.delay_ms(1000_u32);
            status ^= true;

            //dit verschijnt in een van de open terminals in vscode
            hprintln!("Led {:?}", if status { "uit!" } else { "aan!" });
        }
    }

    loop {}
}
