use app::{CLIApp, PiggytagCmd};
use clap::Parser;

mod app;
mod cli;

fn main() {
    let args = cli::Args::parse();
    CLIApp::run_action(args);
}
