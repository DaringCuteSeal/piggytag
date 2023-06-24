use clap::{arg, Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[command(about="Piggytag subcommand", long_about = None)]
pub enum PiggytagSubcommand {
    Show,
    Edit,
}

#[derive(Parser, Debug)]
#[command(author, version=env!("CARGO_PKG_VERSION"), about="A CLI audio metadata editor", long_about = None)]
pub struct Args {
    /** Piggytag subcommand */
    #[command(subcommand)]
    pub subcommand: PiggytagSubcommand,

    /** Audio file name */
    #[arg()]
    pub filename: String,
}
