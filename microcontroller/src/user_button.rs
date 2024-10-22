use cortex_m_semihosting::hprintln;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum BridgeState {
    Open,
    #[default] Closed,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct UserButton;

impl UserButton {
    pub fn bridge_state(state: BridgeState) -> BridgeState {
        match state {
            BridgeState::Open => {
                hprintln!("The bridge is now open.");
                BridgeState::Open
            },
            BridgeState::Closed => BridgeState::Closed
        }
    }
}
