#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

extern crate alloc;

use crate::alloc::boxed::Box;
use baremetal_microcontroller::{color_led::*, LedState};
use core::ops::{Mul, Sub};

use cortex_m_rt::entry;
// Halt on panic
use panic_semihosting as _; // panic handler
use stm32f4::stm32f411::Peripherals as Stm32Peripherals;
use cortex_m::{delay::Delay, Peripherals as CortexPeripherals};

fn gpiod_register(stm32_peripherals: &Stm32Peripherals) {
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

        clock_register(&stm32_peripherals);
        gpiod_register(&stm32_peripherals);

        let mut led_state: Option<Box<dyn LedState>> = Some(Box::new(RedLed));
        
        loop {
            if let Some(led) = led_state.take() {
                led.delay_status(&mut delay);
                led_state = Some(led.next(&stm32_peripherals, &mut delay).unwrap());
            }
        }
    }

    loop {}
}
