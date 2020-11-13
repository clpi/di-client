use clap::{App, Arg, ArgEnum};

#[derive(Clone)]
pub struct ItemCmd {
    pub app: App<'static>,
}

impl ItemCmd {

    pub fn new() -> Self {
        ItemCmd {
            app: App::new("item")
                .short_flag("A".chars().nth(0).unwrap())
        }
    }
}


