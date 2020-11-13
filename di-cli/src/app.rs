use ansi_colours::{AsRGB};
use clap::{App, AppSettings, Arg, ArgSettings,};
use directories_next::{UserDirs, BaseDirs};
use crate::{
    config::DiConfig,
    cmd::{DivCmd, RecordCmd, FieldCmd, TagCmd, ItemCmd  },
};

#[derive(Clone)]
pub struct DivApp{app: App<'static>}

impl DivApp {

    pub fn new () -> Self {
        log::info!("App starting up!");
        let conf = DiConfig::new();
        let app = App::new("div")
            .version(conf.version)
            .author(conf.author)
            .about(conf.about)
            .subcommands(vec![
                ItemCmd::new().app,
                RecordCmd::new().app,
                FieldCmd::new().app,
            ])
            .subcommand(Self::init_stats())
            .arg(Arg::new("help")
                .short("h".chars().next().unwrap())
                .long("help")
                .about("help"))
            .arg(Arg::new("config")
                .short("c".chars().last().unwrap())
                .long("config").about("config"))
            .setting(AppSettings::ColoredHelp);
        Self{app}
    }

    fn handle_input(input: &str) -> () {
        let input = shellwords::split(input).expect("Could not split input");
    }

    fn init_stats() -> App<'static> {
        let stats = App::new("stats")
            .version("0.0.1")
            .args(vec![
                Arg::new("test")
                .short("t".chars().nth(0).unwrap())
                .long("Test")
                .about("Test")
                .takes_value(true)
            ]);
        stats
    }

    pub fn run(self) {
        let matches = self.app.get_matches();
        match matches.occurrences_of("i") {
            1 => { log::info!("Got i"); },
            _ => {},
        }
        if let Some(matches) = matches.subcommand_matches("test") {
            if matches.is_present("debug") {
                log::info!("Debug present");
            } else { log::warn!("No debug") }
        }
    }

}

