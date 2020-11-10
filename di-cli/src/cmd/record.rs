use argh::FromArgs;


#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "record", description=" ")]
pub struct RecordCmd {
    #[argh(switch)]
    /// whether to fooey
    name: String,
    #[argh(option)]
    value: Option<String>,
    #[argh(option)]
    active: Option<bool>,
    #[argh(option)]
    add_item: Option<String>,
    #[argh(option)]
    del_item: Option<String>,
}
