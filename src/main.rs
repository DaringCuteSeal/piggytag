use std::{borrow::Cow, cell::Ref};

use app::PiggytagCmd;
use clap::Parser;

mod app;
mod cli;

fn main() {
    let args = cli::Args::parse();
    match args.subcommand {
        cli::PiggytagSubcommand::Show => app::App::show_meta_from_file(args.filename),
        _ => todo!(),
    }
}
