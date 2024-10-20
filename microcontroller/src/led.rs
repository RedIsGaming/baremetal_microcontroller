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
}
