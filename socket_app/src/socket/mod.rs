mod bindings;
use libloading::{Library, Symbol};

use std::path::PathBuf;

pub use bindings::SmartSocketState;

use bindings::{GetConsumptionWattFn, GetStateFn, SwithOffFn, SwithOnFn};
pub struct SmartSocket(Library);

// #[cfg(debug_assertions)]
// fn lib_path() -> PathBuf {
//     PathBuf::from("..\\target\\debug\\socket_lib.dll")
// }

// #[cfg(not(debug_assertions))]
fn lib_path() -> PathBuf {
    PathBuf::from("socket_lib.dll")
}

impl SmartSocket {
    pub fn new() -> Self {
        Self(unsafe { Library::new(lib_path()).expect("Failed to load dll") })
    }

    pub fn get_state(&self) -> SmartSocketState {
        unsafe {
            let func: Symbol<GetStateFn> = self
                .0
                .get(b"get_state")
                .expect("Failed to load \"get_state\" function from dll");
            func()
        }
    }

    pub fn switch_on(&self) -> SmartSocketState {
        unsafe {
            let func: Symbol<SwithOnFn> = self
                .0
                .get(b"switch_on")
                .expect("Failed to load \"switch_on\" function from dll");
            func()
        }
    }

    pub fn switch_off(&self) -> SmartSocketState {
        unsafe {
            let func: Symbol<SwithOffFn> = self
                .0
                .get(b"switch_off")
                .expect("Failed to load \"switch_off\" function from dll");
            func()
        }
    }

    pub fn get_consumption_watt(&self) -> u64 {
        unsafe {
            let func: Symbol<GetConsumptionWattFn> = self
                .0
                .get(b"get_consumption_watt")
                .expect("Failed to load \"get_consumption_watt\" function from dll");
            func()
        }
    }
}
