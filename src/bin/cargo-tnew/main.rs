//! > Experimental `cargo new` with templates
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

mod cli;

use std::process;

use clap::Parser;

fn main() {
    let args = cli::Command::parse();

    if let Err(err) = args.exec() {
        anstream::eprintln!("Error: {err:?}");

        process::exit(1);
    }
}
