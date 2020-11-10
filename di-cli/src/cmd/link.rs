use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "field", description=" ")]
pub struct LinkCmd {
    #[argh(switch)]
    /// whether to fooey
    name: Option<String>,
    #[argh(switch)]
    /// whether to fooey
    value: Option<String>,
    #[argh(option)]
    from: String,
    #[argh(option)]
    to: String,
    #[argh(option)]
    active: Option<bool>,
}
