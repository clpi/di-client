use super::{ItemCmd, RecordCmd, FieldCmd, UserCmd};
use std::rc::{Weak, Rc};
use argh::{FromArgs, FromArgValue};

pub enum TagTarget {
    Record,
    Item,
    Field,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "tag", description=" ")]
pub struct TagCmd {
    #[argh(switch)]
    /// whether to fooey
    name: String,
    #[argh(option)]
    value: Option<String>,
    #[argh(option, default = "")]
    status: Option<Status>,
    #[argh(subcommand)]
    target: TagTarget,
}

pub enum TagTarget {
    TagItem(TagItemCmd),
    TagRecord(TagRecordCmd),
    TagField(TagFieldCmd),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "item", description=" ")]
pub struct TagItemCmd {
    name: String,
    item: Rc<Item>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "record", description=" ")]
pub struct TagRecordCmd {
    name: String,
    record: Rc<Record>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "field", description=" ")]
pub struct TagFieldCmd {
    name: String,
    field: Rc<Field>,
}
