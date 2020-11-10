pub mod app;
pub mod error;
pub mod util;
pub mod config;

use std::{path::PathBuf, sync::Arc, time::Duration};
use clap::{crate_version, App, AppSettings, Arg};

pub struct DivApp {
    settings: Vec<AppSettings>,
}

fn main() {
    println!("Hello, world!");
}
