use std::{fs::File, io::stdout, path::Path};

use clap::builder::OsStr;
use colored::Colorize;
use lofty::{Tag, TagExt, TaggedFileExt};
use piggytag::{
    error::{error_handler, PiggytagError},
    get_tagged_file, prnt_info,
    structs::AudioMetadata,
};

use crate::cli::{self, Args};

pub struct CLIApp;

pub trait PiggytagCmd {
    /** Do action based on the given subcommand */
    fn run_action(args: Args);

    /** Show audio tag information (pretty-print) */
    fn show_audio_tag_info<P: AsRef<str>>(file_path: P, tag_idx: usize, multiple_files: bool);

    /** List tags available (pretty-print) */
    fn list_tags<P: AsRef<str>>(file_path: P, multiple_files: bool);

    /** Edit or create a new tag. If the given `tag_idx` is `None`, create a new tag instead. */
    fn mutate_tag<P: AsRef<str>>(file_path: P, metadata: &AudioMetadata, tag: &mut Tag);
}

impl PiggytagCmd for CLIApp {
    fn run_action(args: Args) {
        match args.subcommand {
            /* Show tag information */
            cli::PiggytagSubcommand::Show(args) => {
                let multiple_files = args.filename.len() > 1;
                for filename in args.filename {
                    CLIApp::show_audio_tag_info(filename, args.tag_idx - 1, multiple_files);
                }
            }
            /* List available tags */
            cli::PiggytagSubcommand::List(args) => {
                for filename in args.filename {
                    CLIApp::list_tags(filename, false);
                }
            }
            /* Edit tag */
            cli::PiggytagSubcommand::Edit(args) => {
                // XXX do we really need to remake all the metadatas for each file, just to change the filename? refactor later pls my battery's at 10%
                for filename in &args.filename {
                    let metadata = AudioMetadata {
                        filename: Some(filename.clone()),
                        title: args.title.clone(),
                        artist: args.artist.clone(),
                        album: args.album.clone(),
                        track: args.track,
                        year: args.year,
                        comment: args.comment.clone(),
                        genre: args.genre.clone(),
                    };
                    if args.new_tag {
                        let mut tag = Tag::new(lofty::TagType::Id3v2);
                        CLIApp::mutate_tag(filename, &metadata, &mut tag)
                    } else {
                        let tagged_file = match get_tagged_file(filename) {
                            Ok(tagged_file) => tagged_file,
                            Err(err) => error_handler(err, &mut stdout()),
                        };
                        let tags = tagged_file.tags();
                        if tags.len() > 
                        let mut tag = tagged_file.tags()[args.tag_idx].to_owned();
                        CLIApp::mutate_tag(filename, &metadata, &mut tag)
                    }
                }
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
        match AudioMetadata::parse_from_tagged_file(file_path_parsed, &tagged_file, tag_idx) {
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

    fn mutate_tag<P: AsRef<str>>(file_path: P, metadata: &AudioMetadata, tag: &mut Tag) {
        let file_path_parsed = Path::new(file_path.as_ref());
        let tmp_os_str = OsStr::from("unknown");
        let file_path_name = file_path_parsed
            .file_name()
            .unwrap_or(&tmp_os_str)
            .to_str()
            .unwrap_or("unknown");

        metadata.mutate_tag(tag);
        let mut opened_file = match File::open(file_path_parsed) {
            Ok(file) => file,
            Err(err) => error_handler(PiggytagError::Io(err), &mut stdout()),
        };
        match tag.save_to(&mut opened_file) {
            Ok(_) => {}
            Err(err) => error_handler(
                PiggytagError::LoftyError {
                    file_name: (file_path_name.to_owned()),
                    err: (err),
                },
                &mut stdout(),
            ),
        };
    }
}
