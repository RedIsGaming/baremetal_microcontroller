use cortex_m::delay::Delay;
use cortex_m_semihosting::hprintln;

#[derive(Debug, Default)]
pub struct Leds {
    pub status: bool,
    pub delay_time: u32,
}

impl Leds {
    pub fn new(status: bool, delay_time: u32) -> Self {
        Self {
            status,
            delay_time,
        }
    }

    pub fn delay(mut self, delay: &mut Delay) -> Self {
        delay.delay_ms(self.delay_time);
        self.status ^= true;
        hprintln!("The led is now triggered.");
        
        self
    }
}
