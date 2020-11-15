use chrono::{DateTime, Utc};
use clap::{Clap, Arg, ArgEnum, App};

#[derive(Clap, Debug)]
pub struct RecordCmd {
    name: String,
    value: Option<String>
}

pub fn new() -> App<'static> {
    App::new("record")
        .short_flag("R".chars().nth(0).unwrap())
        .about("Define a new record or update an exisitng record")
}
