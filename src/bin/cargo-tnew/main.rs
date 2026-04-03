//! > Experimental `cargo new` with templates
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

use cargo_tnew::util::GlobalContext;

mod cli;
mod tnew;

fn main() {
    let gctx = GlobalContext::new();
    if let Err(err) = cli::main(&gctx) {
        cargo_tnew::exit_with_error(err, &mut gctx.shell());
    }
}
