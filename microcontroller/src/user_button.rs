use cortex_m_semihosting::hprintln;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum UserButton {
    Open,
    #[default] Closed,
}

impl UserButton {
    pub fn bridge_state(&self) -> &Self {
        match *self {
            UserButton::Open => {
                UserButton::open_bridge(&UserButton::Open)
            },
            UserButton::Closed => {
                UserButton::close_bridge(&UserButton::Closed)
            }
        }
    }

    fn open_bridge(&self) -> &Self {
        hprintln!("The bridge is now open.");
        self
    }

    fn close_bridge(&self) -> &Self {
        self
    }
}
