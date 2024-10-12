#![no_std]
#![no_main]
#![allow(unused)]

use cortex_m_rt::entry;
use stm32f4::stm32f411::Peripherals;
use panic_semihosting as _;

fn set_led(peripheral: &Peripherals) {
    peripheral.GPIOD.odr.modify(|_, w| w
        .odr12().set_bit()
        .odr13().set_bit()
        .odr14().set_bit()
        .odr15().set_bit())
}

fn delay() {
    for _ in 0..1000000 { }
}

fn clear_led(peripheral: &Peripherals) {
    peripheral.GPIOD.odr.modify(|_, w| w
        .odr12().clear_bit()
        .odr13().clear_bit()
        .odr14().clear_bit()
        .odr15().clear_bit())
}

#[entry]
unsafe fn main() -> ! {
    //let peripheral = Peripherals::take().unwrap();
    
    //peripheral.RCC.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
    // peripheral.GPIOA.moder.modify(|_, w| w
    //     .moder12().output()
    //     .moder13().output()
    //     .moder14().output()
    //     .moder15().output());

    //set_led(&peripheral);
    
    loop {
        delay();
        //clear_led(&peripheral);
        delay();
        //set_led(&peripheral);
    }
}
