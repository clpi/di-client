use clap::{Arg, ArgEnum, App};

#[derive(Clone)]
pub struct FieldCmd {
    pub app: App<'static>,
}

impl FieldCmd {
    pub fn new() -> Self {
        Self {
            app: App::new("field")
                .short_flag("F".chars().nth(0).unwrap())
        }
    }
}
