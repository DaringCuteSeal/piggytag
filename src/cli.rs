use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about="Show arguments", long_about=None)]
pub struct ShowArgs {
    /** Audio file name */
    #[arg(required = true)]
    pub filename: Vec<String>,

    /** Tag to operate on */
    #[clap(short = 'n', default_value_t = 1)]
    pub tag_idx: usize,
}

#[derive(Debug, Parser)]
#[command(about="Edit arguments", long_about=None)]
pub struct EditArgs {
    /** Audio file name */
    #[arg(required = true)]
    pub filename: Vec<String>,

    /** Create a new tag instead of mutating in-place */
    #[arg(short = 'c', long = "create")]
    pub new_tag: bool,

    /** Tag to operate on */
    #[arg(short = 'n', default_value_t = 1)]
    pub tag_idx: usize,

    /** Set audio title */
    #[arg(long = "title")]
    pub title: Option<String>,

    /** Set audio artist */
    #[arg(long = "artist")]
    pub artist: Option<String>,

    /** Set audio album */
    #[arg(long = "album")]
    pub album: Option<String>,

    /** Set audio artist */
    #[arg(long = "track")]
    pub track: Option<u32>,

    /** Set audio release year */
    #[arg(long = "year")]
    pub year: Option<u32>,

    /** Set audio comment */
    #[arg(long = "comment")]
    pub comment: Option<String>,

    /** Set audio genre */
    #[arg(long = "comment")]
    pub genre: Option<String>,
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
    /** Show audio tag information */
    Show(ShowArgs),

    /** Edit audio tag */
    Edit(EditArgs),

    /** List available tag(s) */
    List(ListArgs),
}

#[derive(Parser, Debug)]
#[command(author, version=env!("CARGO_PKG_VERSION"), about="A CLI audio metadata editor.", long_about = None)]
pub struct Args {
    /** Piggytag subcommand */
    #[clap(subcommand)]
    pub subcommand: PiggytagSubcommand,
}
