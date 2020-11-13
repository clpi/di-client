use clap::{Arg, ArgEnum, App};

#[derive(Clone)]
pub struct RecordCmd {
    pub app: App<'static>,
}

impl RecordCmd {
    pub fn new() -> Self {
        Self {
            app: App::new("record")
                .short_flag("R".chars().nth(0).unwrap())
        }
    }
}
