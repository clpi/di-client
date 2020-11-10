
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "field", description=" ")]
pub struct FieldCmd {
    #[argh(switch)]
    /// whether to fooey
    name: String,
    #[argh(option)]
    value: Option<String>,
    #[argh(option)]
    field_type: Option<String>,
    #[argh(option)]
    active: Option<bool>,
    #[argh(option)]
    add_to_item: Option<String>,
}
