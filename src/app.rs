use std::{io::stdout, path::Path};

use piggytag::{
    error::{error_handler, PiggytagError},
    get_tagged_file, prnt_info,
    structs::AudioMetadata,
};

use crate::cli::PiggytagSubcommand;

pub struct App;

pub trait PiggytagCmd {
    fn show_meta_from_file<P: AsRef<str>>(file_path: P);
    fn list_tags<P: AsRef<str>>(file_path: P);
}

impl PiggytagCmd for App {
    fn show_meta_from_file<P: AsRef<str>>(file_path: P) {
        let binding = file_path.as_ref();
        let file_path_parsed = Path::new(&binding);
        let tagged_file = match get_tagged_file(file_path_parsed) {
            Ok(tagged_file) => tagged_file,
            Err(err) => error_handler(err, &mut stdout()),
        };
        match AudioMetadata::parse_from_tagged_file(file_path_parsed, tagged_file) {
            Some(metadata) => println!("{}", metadata),
            None => prnt_info("Audio has no tag(s)."),
        };
        std::process::exit(0);
    }

    fn list_tags<P: AsRef<str>>(file_path: P) {}
}
