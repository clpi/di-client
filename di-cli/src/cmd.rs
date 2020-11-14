pub mod item;
pub mod field;
pub mod record;
pub mod tag;
pub mod link;
pub mod stats;
pub mod user;

use clap::{App, Arg, AppSettings};
use crate::config::DiConfig;

pub trait Cmd {
    fn help_string() -> String;
    fn match_sub(input: &str) -> ();
}

/// Top-level command.
pub struct Div {
    pub app: App<'static>,
}

impl Div {

    pub fn new() -> Self {
        let conf = DiConfig::new();
        Self {
            app: App::new("div")
                .bin_name("div")
                .version(conf.version)
                .author(conf.author)
                .about(conf.about)
                .subcommands(vec![
                    item::new(),
                    record::new(),
                    field::new(),
                    stats::new(),
                ])
                .args(vec![
                    Self::config(),
                    Self::help(),
                ])
                .setting(AppSettings::ColoredHelp)
        }
    }

    pub fn invoke(input: &str) -> () {
        let input = shellwords::split(input).expect("Could not split input");
    }

    pub fn config() -> Arg<'static> {
        Arg::new("config")
            .short("c".chars().last().unwrap())
            .long("config").about("config")
    }

    pub fn help() -> Arg<'static> {
        Arg::new("help")
            .short("h".chars().next().unwrap())
            .long("help")
            .about("help")
    }
}
