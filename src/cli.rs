use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about="Show arguments", long_about=None)]
pub struct ShowArgs {
    /** Audio file name */
    #[arg(required = true)]
    pub filename: Vec<String>,

    /** Tag to operate on */
    #[clap(short = 'n', default_value_t = 1, global = true)]
    pub tag_idx: usize,
}

#[derive(Debug, Parser)]
#[command(about="Edit arguments", long_about=None)]
pub struct EditArgs {
    /** Audio file name */
    #[arg(required = true)]
    pub filename: Vec<String>,

    /** Tag to operate on */
    #[clap(short = 'n', default_value_t = 1, global = true)]
    pub tag_idx: usize,
}

#[derive(Debug, Parser)]
#[command(about="List arguments", long_about=None)]
pub struct ListArgs {
    /** Audio file name */
    #[arg(required = true)]
    pub filename: Vec<String>,
}

#[derive(Debug, Subcommand)]
#[command(about="Piggytag subcommands", long_about=None)]
pub enum PiggytagSubcommand {
    /** Show tag(s) information */
    Show(ShowArgs),

    /** Edit tag(s) */
    Edit(EditArgs),

    /** List available tag(s) */
    List(ListArgs),
}

#[derive(Parser, Debug)]
#[command(author, version=env!("CARGO_PKG_VERSION"), about="A CLI audio metadata editor", long_about = None)]
pub struct Args {
    /** Piggytag subcommand */
    #[clap(subcommand)]
    pub subcommand: PiggytagSubcommand,
}
