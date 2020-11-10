
use argh::FromArgs;
#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "item", description=" ")]
pub struct ItemCmd {
    #[argh(option)]
    /// how many x
    name: String,
    #[argh(option)]
    value: Option<String>,
    #[argh(option)]
    active: Option<bool>,
    #[argh(option)]
    add_field: Option<String>,
    #[argh(option)]
    del_field: Option<String>,
    #[argh(option)]
    add_to_record: Option<String>,
}
