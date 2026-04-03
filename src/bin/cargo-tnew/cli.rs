use cargo_tnew::util::GlobalContext;
use cargo_tnew::CargoResult;
use clap::Command;

pub(crate) fn main(gctx: &GlobalContext) -> CargoResult<()> {
    let args = match cli().try_get_matches() {
        Ok(args) => args,
        Err(clap_err) => {
            let exit_code = if clap_err.use_stderr() { 1 } else { 0 };
            let _ = clap_err.print();
            std::process::exit(exit_code)
        }
    };
    match args.subcommand() {
        Some(("new", args)) => crate::tnew::exec(args, gctx)?,
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
