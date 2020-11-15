use chrono::{DateTime, Utc};
use clap::{Clap, Arg, ArgEnum, App};

/// Subcommand which allows for configuration from the CLI
#[derive(Clap, Debug)]
pub struct ConfigCmd {
    #[clap(
        long="key",
        short='k',
        about="Name of the configuration key to be modified"
    )]
    key: String,
    #[clap(
        long="value",
        short='v',
        about="The new value of the configuration item"
    )]
    value: String,
}
