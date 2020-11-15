use chrono::{DateTime, Utc};
use clap::{Clap, App, Arg, ArgEnum};

#[derive(Clap, Debug, PartialEq)]
pub struct ItemCmd {
    #[clap(
        long="name",
        short='n',
        about="name of item"
    )]
    name: String,
    #[clap(
        long="value",
        short='v',
        about="Optional value of item"
    )]
    value: Option<String>,
}

/// Item subcommand
pub fn new() -> App<'static> {
    App::new("item")
        .short_flag('I')
        .about("Define a new item or update an exisitng item")
        .args(vec![
            delete(),
        ])
}

pub fn delete() -> Arg<'static> {
    Arg::new("delete")
        .short('d')
        .about("Delete item from memory")
}


