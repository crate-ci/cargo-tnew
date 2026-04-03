use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;

use cargo_tnew::ops::cargo_new::{self, NewOptions, VersionControl};
use cargo_tnew::util::GlobalContext;
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
        .arg(
            opt("edition", "Edition to set for the crate generated")
                .value_parser(["2015", "2018", "2021", "2024"])
                .value_name("YEAR"),
        )
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

pub(crate) fn exec(args: &ArgMatches, gctx: &GlobalContext) -> CargoResult<()> {
    let opts = new_options(args, gctx)?;
    cargo_new::new(&opts, gctx)
}

fn new_options(args: &ArgMatches, gctx: &GlobalContext) -> CargoResult<NewOptions> {
    let version_control = args
        .get_one::<String>("vcs")
        .map(|vcs| vcs.parse::<VersionControl>())
        .transpose()?;

    NewOptions::new(
        version_control,
        args.get_flag("bin"),
        args.get_flag("lib"),
        gctx.cwd()
            .join(args.get_one::<String>("path").expect("required by clap")),
        args.get_one::<String>("name").cloned(),
        args.get_one::<String>("edition").cloned(),
        args.get_one::<String>("registry").cloned(),
    )
}
