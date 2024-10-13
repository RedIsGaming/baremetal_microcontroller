#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use cortex_m_rt::entry;
// Halt on panic
use panic_semihosting as _; // panic handler
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
unsafe fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {       
        //GPIOD ophalen
        let gpiod = dp.GPIOD.split();
        //pd12 is pin type
        let mut led = gpiod.pd12.into_push_pull_output();

        //Klok instellen
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            // On for 1s, off for 1s.
            led.toggle();
            delay.delay_ms(1000_u32);
        }
    }

    loop {}
}
