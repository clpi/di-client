use chrono::{DateTime, Utc};
use clap::{Arg, ArgEnum, App};

pub fn new() -> App<'static> {
    App::new("record")
        .short_flag("R".chars().nth(0).unwrap())
        .about("Define a new record or update an exisitng record")
}
