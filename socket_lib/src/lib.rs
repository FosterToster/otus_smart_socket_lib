mod socket;

use socket::{SmartSocket, SmartSocketState};

lazy_static::lazy_static! {
    static ref SOCKET: SmartSocket = SmartSocket::default();
}

#[no_mangle]
pub extern "C" fn get_state() -> SmartSocketState {
    SOCKET.get_state()
}

#[no_mangle]
pub extern "C" fn switch_on() -> SmartSocketState {
    SOCKET.set_state(SmartSocketState::Enabled);
    SOCKET.get_state()
}

#[no_mangle]
pub extern "C" fn switch_off() -> SmartSocketState {
    SOCKET.set_state(SmartSocketState::Disabled);
    SOCKET.get_state()
}

#[no_mangle]
pub extern "C" fn get_consumption_watt() -> u64 {
    SOCKET.power_consumption_watt()
}
