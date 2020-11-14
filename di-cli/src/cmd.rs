pub mod item;
pub mod field;
pub mod record;
pub mod tag;
pub mod link;
pub mod stats;

use clap::{App, Arg, AppSettings};
use crate::config::DiConfig;

pub use self::{
    field::FieldCmd,
    item::ItemCmd,
    record::RecordCmd,
    link::LinkCmd,
    tag::TagCmd,
    stats::StatsCmd,
};


pub trait Cmd {
    fn help_string() -> String;
    fn match_sub(input: &str) -> ();
}

/// Top-level command.
pub struct DivCmd {
    pub app: App<'static>,
}

impl DivCmd {

    pub fn new() -> Self {
        let conf = DiConfig::new();
        Self {
            app: App::new("div")
            .version(conf.version)
            .author(conf.author)
            .about(conf.about)
            .subcommands(vec![
                ItemCmd::new().app,
                RecordCmd::new().app,
                FieldCmd::new().app,
            ])
            .subcommand(StatsCmd::new().app)
            .arg(Arg::new("help")
                .short("h".chars().next().unwrap())
                .long("help")
                .about("help"))
            .arg(Arg::new("config")
                .short("c".chars().last().unwrap())
                .long("config").about("config"))
            .setting(AppSettings::ColoredHelp)
        }
    }

    pub fn invoke(input: &str) -> () {
        let input = shellwords::split(input).expect("Could not split input");
    }
}
