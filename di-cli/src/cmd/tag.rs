use chrono::{DateTime, Utc};
use clap::{Clap, Arg, ArgEnum, App};

#[derive(Clap, Debug)]
pub struct TagCmd {
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
    #[clap(
        long="value",
        short='k',
        about="The kind of resource targeted by the tag"
    )]
    target_kind: Option<String>,
    #[clap(
        long="value",
        short='t',
        about="The name of the target resource"
    )]
    target_name: Option<String>,
    #[clap(
        long="priority",
        short='p',
        about="The status of this tag"
    )]
    status: i32,
}

impl TagCmd {

    pub fn new_app() -> App<'static> {
        return App::new("tag")
            .about("Tag an resource with a key and (optional) value pair")
            .short("t")
    }

    pub fn get_matches() -> () {

    }
}
