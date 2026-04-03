//! > Experimental `cargo new` with templates
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

use std::process;

mod cli;
mod tnew;

fn main() {
    if let Err(err) = cli::main() {
        anstream::eprintln!("error: {err:?}");

        process::exit(1);
    }
}
