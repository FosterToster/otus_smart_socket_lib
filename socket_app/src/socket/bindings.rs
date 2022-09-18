use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum SmartSocketState {
    Enabled = 1,
    Disabled = 0,
}

impl FromStr for SmartSocketState {
    type Err = type_cli::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "on" => Ok(SmartSocketState::Enabled),
            "off" => Ok(SmartSocketState::Disabled),
            _ => Err(type_cli::Error::ExtraArg(format!(
                "Invalid socket state \"{}\". Use \"on\" or \"off\"",
                s
            ))),
        }
    }
}

impl ToString for SmartSocketState {
    fn to_string(&self) -> String {
        match self {
            Self::Enabled => "Enabled".to_string(),
            Self::Disabled => "Disabled".to_string(),
        }
    }
}

pub type GetStateFn = unsafe extern "C" fn() -> SmartSocketState;
pub type SwithOnFn = unsafe extern "C" fn() -> SmartSocketState;
pub type SwithOffFn = unsafe extern "C" fn() -> SmartSocketState;
pub type GetConsumptionWattFn = unsafe extern "C" fn() -> u64;
