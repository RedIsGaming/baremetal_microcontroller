use cortex_m::delay::Delay;

#[derive(Debug, Default)]
pub struct Leds {
    status: bool,
    delay_time: u32,
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
        
        self
    }
}
