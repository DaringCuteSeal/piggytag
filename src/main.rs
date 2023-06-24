use std::{borrow::Cow, cell::Ref};

use app::PiggytagCmd;
use clap::Parser;
use piggytag::prnt_sep;

mod app;
mod cli;

fn main() {
    let args = cli::Args::parse();
    match args.subcommand {
        cli::PiggytagSubcommand::Show(show_args) => {
            let multiple_files = show_args.filename.len() > 1;
            for filename in show_args.filename {
                app::CLIApp::show_audio_tag_info(filename, show_args.tag_idx - 1, multiple_files);
            }
        }
        cli::PiggytagSubcommand::List(list_args) => {
            for filename in list_args.filename {
                app::CLIApp::list_tags(filename, false);
            }
        }
        _ => todo!(),
    }
}
