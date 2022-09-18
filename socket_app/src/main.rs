mod cli;
mod socket;

use cli::SocketCli;
use socket::{SmartSocket, SmartSocketState};
use type_cli::CLI;

fn main() {
    match SocketCli::process() {
        SocketCli::State { get, set, noformat } => {
            let mut template = "{}";
            let result: String;

            let socket = SmartSocket::new();
            if let (false, Some(state)) = (get, set) {
                template = "Socket state been set to {}";
                match state {
                    SmartSocketState::Enabled => {
                        result = socket.switch_on().to_string();
                    }
                    SmartSocketState::Disabled => {
                        result = socket.switch_off().to_string();
                    }
                }
            } else if let (true, None) = (get, set) {
                template = "Current socket state is {}";
                result = socket.get_state().to_string();
            } else {
                result =
                    "Ambigolous arguments. Consider using --get and --set separately.".to_string()
            }

            if noformat {
                println!("{}", result);
            } else {
                println!("{}", template.replace("{}", &result));
            }
        }
        SocketCli::Consumption { noformat } => {
            if noformat {
                println!("{}", SmartSocket::new().get_consumption_watt());
            } else {
                println!(
                    "Current socket power consumption is {}W",
                    SmartSocket::new().get_consumption_watt()
                );
            }
        }
    }
}
