use clap::{App, Arg};

pub fn new() -> App<'static> {
        App::new("stats")
            .version("0.0.1")
            .args(vec![
                test(),
            ])
    }

pub fn test() -> Arg<'static> {
    Arg::new("test")
        .short("t".chars().nth(0).unwrap())
        .long("Test")
        .about("Test")
        .takes_value(true)
}
