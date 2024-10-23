#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

extern crate alloc;

use crate::alloc::boxed::Box;
use baremetal_microcontroller::{colors::led::*, registers::{clock, gpiod}, LedState};
use cortex_m_semihosting::hprintln;
use core::ops::{Mul, Sub};

use cortex_m_rt::entry;
// Halt on panic
use panic_semihosting as _; // panic handler
use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use cortex_m::{delay::Delay, Peripherals as CortexPeripherals};

#[entry]
unsafe fn main() -> ! {
    if let Some(stm32_peripherals) = Stm32Peripherals::take() {
        let cortex_peripherals = CortexPeripherals::take().unwrap();

        // Create a delay abstraction based on SysTick
        let mut delay = Delay::new(cortex_peripherals.SYST, 0x7A1200_u32.mul(2).sub(1));

        clock::clock_register(&stm32_peripherals);
        gpiod::gpiod_register(&stm32_peripherals);

        let mut led_state: Option<Box<dyn LedState>> = Some(Box::new(RedLed));
        
        loop {
            if let Some(led) = led_state.take() {
                led.delay_status(&mut delay);

                match led.next(&stm32_peripherals, &mut delay) {
                    Ok(led_color) => led_state = Some(led_color),
                    Err(err) => {
                        hprintln!("An unexpected problem has occurred: {}", err);
                        led_state = Some(Box::new(BlueLed))
                    }
                }
            }
        }
    }

    loop {}
}
