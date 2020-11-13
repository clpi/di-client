use ansi_colours::{AsRGB};
use clap::{App, AppSettings, Arg, ArgSettings,};
use directories_next::{UserDirs, BaseDirs};
use crate::{
    config::DiConfig,
    cmd::{DivCmd, RecordCmd, FieldCmd, TagCmd, ItemCmd, StatsCmd },
};

#[derive(Clone)]
pub struct DivApp{app: App<'static>}

impl DivApp {

    pub fn new () -> Self {
        log::info!("App starting up!");
        let conf = DiConfig::new();
        let app = DivCmd::new().app;
    }

    fn handle_input(input: &str) -> () {
        let input = shellwords::split(input).expect("Could not split input");
    }

    fn init_stats() -> App<'static> {
        let stats =         stats
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

