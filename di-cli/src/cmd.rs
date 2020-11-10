pub mod item;
pub mod field;
pub mod record;

pub mod user;
pub mod tag;
pub mod link;

use argh::FromArgs;
pub use self::{
    field::FieldCmd,
    item::ItemCmd,
    record::RecordCmd,
    user::UserCmd,
    link::LinkCmd,
    tag::{TagCmd, TagItemCmd, TagFieldCmd, TagRecordCmd},
};

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
pub struct Div {
    #[argh(switch, short = 'n')]
    init: bool,
    #[argh(option, short = 'h')]
    help: Option<String>,
    #[argh(switch, short = 'v')]
    verbose: bool,
    #[argh(subcommand)]
    /// div profile ...
    element: DivEement,
    #[argh(subcommand)]
    /// div profile ...
    profile: ProfileCmd,
}

#[derive(FromArgs, PartialEq, Debug)]
pub enum DivElement {
    Item(ItemCmd),
    Field(FieldCmd),
    Record(RecordCmd)
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name="user")]
pub enum DivUser {
    User(UserCmd),
    Login,
}

