use super::socket::SmartSocketState;
use type_cli::CLI;

#[derive(CLI)]
pub enum SocketCli {
    #[help = "Maintain socket state"]
    State {
        #[flag]
        #[help = "Get current socket state"]
        get: bool,

        #[named]
        #[optional]
        #[help = "Set socket state"]
        set: Option<SmartSocketState>,

        #[flag]
        #[help = "Disable output formatting"]
        noformat: bool,
    },

    #[help = "Get current power consumption"]
    Consumption {
        #[flag]
        #[help = "Disable output formatting"]
        noformat: bool,
    },
}
