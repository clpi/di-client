use ansi_colours::{AsRGB};
use clap::App;


use crate::{ config::DiConfig, cmd, };

#[derive(Clone)]
pub struct DivApp{app: App<'static>}

impl DivApp {

    pub fn new () -> Self {
        log::info!("App starting up!");
        let _conf = DiConfig::new();
        let app = cmd::Div::new().app;
        Self { app }
    }

    fn _handle_input(input: &str) -> () {
        let _input = shellwords::split(input).expect("Could not split input");
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

