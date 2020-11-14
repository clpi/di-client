use chrono::{DateTime, Utc};
use clap::{Arg, ArgEnum, App};

pub fn new() -> App<'static> {
    App::new("field")
        .short_flag("F".chars().nth(0).unwrap())
        .about("Define a new field or update an exisitng field")
}
