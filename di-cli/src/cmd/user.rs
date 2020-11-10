use argh::FromArgs;


#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "profile", description=" ")]
pub struct UserCmd {
    #[argh(switch)]
    /// upd
    update: bool,
    #[argh(switch)]
    login: bool,
}
