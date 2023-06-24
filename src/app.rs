use std::{io::stdout, path::Path};

use clap::builder::OsStr;
use lofty::TaggedFileExt;
use piggytag::{
    error::{error_handler, PiggytagError},
    get_tagged_file, prnt_info, prnt_sep,
    structs::AudioMetadata,
};

use crate::cli::PiggytagSubcommand;

pub struct CLIApp;

pub trait PiggytagCmd {
    /** Show audio tag information (pretty-print) */
    fn show_audio_tag_info<P: AsRef<str>>(file_path: P, tag_idx: usize, multiple_files: bool);

    /** List tags available (pretty-print) */
    fn list_tags<P: AsRef<str>>(file_path: P, multiple_files: bool);
}

impl PiggytagCmd for CLIApp {
    fn show_audio_tag_info<P: AsRef<str>>(file_path: P, tag_idx: usize, multiple_files: bool) {
        let file_path_parsed = Path::new(file_path.as_ref());
        let tagged_file = match get_tagged_file(file_path_parsed) {
            Ok(tagged_file) => tagged_file,
            Err(err) => error_handler(err, &mut stdout()),
        };
        match AudioMetadata::parse_from_tagged_file(file_path_parsed, tagged_file, tag_idx) {
            Some(metadata) => println!("{}", metadata),
            None => prnt_info("Audio has no tag(s)."),
        };
        if multiple_files {
            prnt_sep()
        }
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
            std::cmp::Ordering::Equal => {
                prnt_info(format!("file {}: {}", file_path_name, "1 tag available"))
            }
            std::cmp::Ordering::Greater => prnt_info(format!(
                "file {}: {} tags available",
                file_path_name,
                tagged_file.tags().len()
            )),
            std::cmp::Ordering::Less => {
                prnt_info(format!("file {}: no tag(s) available", file_path_name))
            }
        }
    }
}
