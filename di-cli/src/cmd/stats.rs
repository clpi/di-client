use clap::{App, Arg};

pub struct StatsCmd {
    pub app: App<'static>,
}

impl StatsCmd {
    pub fn new() -> Self {
        Self { app:  App::new("stats")
            .version("0.0.1")
            .args(vec![
                Arg::new("test")
                .short("t".chars().nth(0).unwrap())
                .long("Test")
                .about("Test")
                .takes_value(true)
            ])
        }
    }
}
