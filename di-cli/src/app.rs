use crate::{
    config::DiConfig,
    cmd::{Div, RecordCmd, FieldCmd, TagCmd,   },
};
use argh::FromArgs;

pub struct DivApp {
    cmd: Div,
}


impl DivApp {

    pub fn run() -> () {

    }
}

    /* NOTE For using Clap
use clap::{App, AppSettings, Arg, ArgSettings,};

#[derive(Clone)]
pub struct DiApp {
    app: App<'static>,
}

impl DiApp {

    pub fn new () -> Self {
        log::info!("App starting up!");
        let conf = DiConfig::new();
        let app = App::new("div")
            .version(conf.version)
            .author(conf.author)
            .about(conf.about)
            .args(vec![
                ItemCmd::arg(),
                RecordCmd::arg(),
                FieldCmd::arg(),
            ])
            .subcommand(Self::init_stats());
        Self{ app  }
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
    */
}

