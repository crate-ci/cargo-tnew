use cargo_tnew::CargoResult;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub(crate) enum Command {
    Tnew,
}

impl Command {
    pub(crate) fn exec(self) -> CargoResult<()> {
        match self {
            Self::Tnew => Ok(()),
        }
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Command::command().debug_assert();
}
