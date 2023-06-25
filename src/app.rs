use std::{io::stdout, path::Path};

use clap::builder::OsStr;
use colored::Colorize;
use lofty::TaggedFileExt;
use piggytag::{error::error_handler, get_tagged_file, prnt_info, structs::AudioMetadata};

use crate::cli::{self, Args};

pub struct CLIApp;

pub trait PiggytagCmd {
    /** Do action based on the given subcommand */
    fn run_action(args: Args);

    /** Show audio tag information (pretty-print) */
    fn show_audio_tag_info<P: AsRef<str>>(file_path: P, tag_idx: usize, multiple_files: bool);

    /** List tags available (pretty-print) */
    fn list_tags<P: AsRef<str>>(file_path: P, multiple_files: bool);
}

impl PiggytagCmd for CLIApp {
    fn run_action(args: Args) {
        match args.subcommand {
            /* Show tag information */
            cli::PiggytagSubcommand::Show(show_args) => {
                let multiple_files = show_args.filename.len() > 1;
                for filename in show_args.filename {
                    CLIApp::show_audio_tag_info(filename, show_args.tag_idx - 1, multiple_files);
                }
            }
            /* List available tags */
            cli::PiggytagSubcommand::List(list_args) => {
                for filename in list_args.filename {
                    CLIApp::list_tags(filename, false);
                }
            }
            /* Edit tag */
            cli::PiggytagSubcommand::Edit(edit_args) => {
                todo!();
            }
        }
    }

    fn show_audio_tag_info<P: AsRef<str>>(file_path: P, tag_idx: usize, _multiple_files: bool) {
        let file_path_parsed = Path::new(file_path.as_ref());
        let tmp_os_str = OsStr::from("unknown");
        let file_path_name = file_path_parsed
            .file_name()
            .unwrap_or(&tmp_os_str)
            .to_str()
            .unwrap_or("unknown");

        let tagged_file = match get_tagged_file(file_path_parsed) {
            Ok(tagged_file) => tagged_file,
            Err(err) => error_handler(err, &mut stdout()),
        };
        match AudioMetadata::parse_from_tagged_file(file_path_parsed, tagged_file, tag_idx) {
            Some(metadata) => println!("{}", metadata),
            None => prnt_info(format!(
                "file {}: {}",
                file_path_name.bold(),
                "No tag information available."
            )),
        };
    }

    fn list_tags<P: AsRef<str>>(file_path: P, _multiple_files: bool) {
        let file_path_parsed = Path::new(file_path.as_ref());
        let tmp_os_str = OsStr::from("unknown");
        let file_path_name = file_path_parsed
            .file_name()
            .unwrap_or(&tmp_os_str)
            .to_str()
            .unwrap_or("unknown");
        let tagged_file = match get_tagged_file(file_path_parsed) {
            Ok(tagged_file) => tagged_file,
            Err(err) => error_handler(err, &mut stdout()),
        };
        let tags_count = tagged_file.tags().len();
        match tags_count.cmp(&1) {
            std::cmp::Ordering::Equal => prnt_info(format!(
                "file {}: {}",
                file_path_name.bold(),
                "1 tag available"
            )),
            std::cmp::Ordering::Greater => prnt_info(format!(
                "file {}: {} tags available",
                file_path_name.bold(),
                tagged_file.tags().len()
            )),
            std::cmp::Ordering::Less => prnt_info(format!(
                "file {}: no tag(s) available",
                file_path_name.bold()
            )),
        }
    }
}
