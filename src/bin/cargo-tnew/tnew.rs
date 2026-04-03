use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;

use cargo_tnew::CargoResult;

pub(crate) fn cli() -> Command {
    Command::new("new")
        .alias("tnew")
        .about("Create a new cargo package at <path>")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .action(ArgAction::Set)
                .required(true),
        )
        .arg(
            opt(
                "vcs",
                "Initialize a new repository for the given version \
                 control system, overriding \
                 a global configuration.",
            )
            .value_name("VCS")
            .value_parser(["git", "hg", "pijul", "fossil", "none"]),
        )
        .arg(flag("bin", "Use a binary (application) template [default]"))
        .arg(flag("lib", "Use a library template"))
        .arg(opt("edition", "Edition to set for the crate generated").value_name("YEAR"))
        .arg(
            opt(
                "name",
                "Set the resulting package name, defaults to the directory name",
            )
            .value_name("NAME"),
        )
        .arg(opt("registry", "Registry to use").value_name("REGISTRY"))
}

pub(crate) fn flag(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name)
        .long(name)
        .help(help)
        .action(ArgAction::SetTrue)
}

pub(crate) fn opt(name: &'static str, help: &'static str) -> Arg {
    Arg::new(name).long(name).help(help).action(ArgAction::Set)
}

pub(crate) fn exec(_args: &ArgMatches) -> CargoResult<()> {
    Ok(())
}
