//! > Experimental `cargo new` with templates

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

pub mod core;
pub mod ops;
pub mod util;

pub use util::errors::*;

pub fn display_warning_with_error(
    warning: &str,
    err: &Error,
    shell: &mut cargo_util_terminal::Shell,
) {
    drop(shell.warn(warning));
    drop(writeln!(shell.err()));
    drop(writeln!(shell.err(), "{err}"));
}

pub fn exit_with_error(error: anyhow::Error, shell: &mut cargo_util_terminal::Shell) -> ! {
    tracing::debug!("exit_with_error; err={:?}", error);

    let exit_code = 101;
    let _ = shell.error(error);

    std::process::exit(exit_code)
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
