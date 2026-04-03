use cargo_tnew::CargoResult;
use clap::Command;

pub(crate) fn main() -> CargoResult<()> {
    let args = cli().get_matches();
    match args.subcommand() {
        Some(("new", args)) => crate::tnew::exec(args)?,
        _ => unreachable!(),
    }
    Ok(())
}

pub(crate) fn cli() -> Command {
    Command::new("cargo")
        .bin_name("cargo")
        .styles(clap_cargo::style::CLAP_STYLING)
        .subcommand_required(true)
        .subcommand(crate::tnew::cli())
}

#[test]
fn verify_cli() {
    cli().debug_assert();
}
