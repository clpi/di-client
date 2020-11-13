pub mod item;
pub mod field;
pub mod record;
pub mod tag;
pub mod link;

pub use self::{
    field::FieldCmd,
    item::ItemCmd,
    record::RecordCmd,
    link::LinkCmd,
    tag::TagCmd,
};


pub trait Cmd {
    fn help_string() -> String;
    fn match_sub(input: &str) -> ();
}

/// Top-level command.
pub struct DivCmd {
}

impl DivCmd {

    pub fn invoke(input: &str) -> DivCmd {
        let input = shellwords::split(input).expect("Could not split input");
        DivCmd {  }
    }
}
