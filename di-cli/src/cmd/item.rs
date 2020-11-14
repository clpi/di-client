use chrono::{DateTime, Utc};
use clap::{App, Arg, ArgEnum};

pub fn new() -> App<'static> {
    App::new("item")
        .short_flag("I".chars().nth(0).unwrap())
        .about("Define a new item or update an exisitng item")
        .args(vec![
            delete(),
        ])
}

pub fn delete() -> Arg<'static> {
    Arg::new("delete")
        .short("d".chars().nth(0).unwrap())
        .about("Delete item from memory")
}


