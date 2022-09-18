use std::{path::PathBuf, sync::Mutex};

const DATABASE_FILENAME: &str = "imadatabaseiswear.db";

#[derive(Clone, Copy)]
#[repr(C)]
pub enum SmartSocketState {
    Enabled = 1,
    Disabled = 0,
}

impl Default for SmartSocketState {
    fn default() -> Self {
        Self::Disabled
    }
}

pub struct SmartSocket(pub Mutex<SmartSocketState>);

impl Default for SmartSocket {
    fn default() -> Self {
        if std::path::PathBuf::from(DATABASE_FILENAME).exists() {
            Self(Mutex::new(SmartSocketState::Enabled))
        } else {
            Self(Mutex::new(SmartSocketState::Disabled))
        }
    }
}

impl SmartSocket {
    pub fn get_state(&self) -> SmartSocketState {
        *self.0.lock().unwrap()
    }

    pub fn set_state(&self, state: SmartSocketState) {
        let mut guard = self.0.lock().unwrap();
        match state {
            SmartSocketState::Enabled => {
                std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(DATABASE_FILENAME)
                    .expect("Failed to create database file");
            }
            SmartSocketState::Disabled => {
                if PathBuf::from(DATABASE_FILENAME).exists() {
                    std::fs::remove_file(DATABASE_FILENAME)
                        .expect("Failed to remove database file");
                };
            }
        };

        *guard = state
    }

    pub fn power_consumption_watt(&self) -> u64 {
        if let SmartSocketState::Enabled = self.get_state() {
            1024
        } else {
            0
        }
    }
}
