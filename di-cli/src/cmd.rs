pub mod item;
pub mod field;
pub mod record;
pub mod tag;
pub mod link;
pub mod stats;
pub mod user;

use clap::Clap;
use clap::{App, Arg, AppSettings, Subcommand};
use crate::config::Config;

pub use self::{
    record::RecordCmd,
    item::ItemCmd,
    field::FieldCmd,
};

#[derive(Clap, Debug)]
pub struct DivCmd {
    #[clap(subcommand)]
    pub record: DivOpts,
    pub item: String,
    pub field: String,

    pub user: String,
}


#[derive(Clap, Debug)]
pub enum DivOpts {
    #[clap(name="record", alias="r", arg_enum)]
    Record(RecordCmd),
    #[clap(name="item", alias="i", arg_enum)]
    Item(ItemCmd),
    #[clap(name="field", alias="f", arg_enum)]
    Field(FieldCmd),
}

/// Top-level command.
pub struct Div {
    pub app: App<'static>,
}

impl Div {

pub fn new() -> Self {
        let config = Config::new();
        Self {
            app: App::new(clap::crate_name!())
                .bin_name("div")
                .version(clap::crate_version!())
                .author(clap::crate_authors!())
                .about(clap::crate_description!())
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

pub trait Cmd {
    fn help_string() -> String;
    fn match_sub(input: &str) -> ();
}

