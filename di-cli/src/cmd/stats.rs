use clap::{Clap, App, Arg};

#[derive(Clap, Debug)]
pub struct StatsCmd {
    #[clap(
        long="name",
        short='n',
        about="Name of the tag (tag key)"
    )]
    name: String,
    #[clap(
        long="value",
        short='v',
        about="Optional value of the tag (tag value)"
    )]
    value: Option<String>,
}

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
