use std::path::Path;

use clap::builder::OsStr;
use colored::{Color, Colorize};
use error::PiggytagError;
use lofty::{Probe, TaggedFile};

pub mod error;
pub mod structs;

/** Default string shown when a value is unknown */
const DEFAULT_UNKNOWN_VAL_STR: &str = "unknown";

/** Default separator */
const DEFAULT_SEPARATOR: &str = "----";

/** Get a human-readable key-value pair */
pub fn get_formatted_key_val<T: AsRef<str>, S: ToString>(
    key: T,
    val: Option<S>,
    val_color: Color,
) -> String {
    let val_print = match val {
        Some(val) => val.to_string().color(val_color).italic().to_string(),
        None => DEFAULT_UNKNOWN_VAL_STR
            .to_owned()
            .dimmed()
            .italic()
            .to_string(),
    };
    let mut key_print = key.as_ref().to_owned();
    key_print.push(':');
    format!("{} {}\n", key_print.bold(), val_print)
}

/** Get audio tag */
pub fn get_tagged_file<T: AsRef<Path>>(filename: T) -> Result<TaggedFile, PiggytagError> {
    let file_path_parsed = Path::new(filename.as_ref());
    let tmp_os_str = OsStr::from("unknown");
    let _file_path_name = file_path_parsed
        .file_name()
        .unwrap_or(&tmp_os_str)
        .to_str()
        .unwrap_or("unknown");

    let result = match Probe::open(filename.as_ref()) {
        Ok(res) => res,
        Err(err) => {
            let file_path_parsed = Path::new(filename.as_ref());
            let tmp_os_str = OsStr::from("unknown");
            let file_path_name = file_path_parsed
                .file_name()
                .unwrap_or(&tmp_os_str)
                .to_str()
                .unwrap_or("unknown");
            return Err(PiggytagError::LoftyError {
                file_name: file_path_name.to_owned(),
                err,
            });
        }
    };

    match result.guess_file_type() {
        Ok(result) => match result.read() {
            Ok(file) => Ok(file),
            Err(err) => {
                let file_path_parsed = Path::new(filename.as_ref());
                let tmp_os_str = OsStr::from("unknown");
                let file_path_name = file_path_parsed
                    .file_name()
                    .unwrap_or(&tmp_os_str)
                    .to_str()
                    .unwrap_or("unknown");

                Err(PiggytagError::LoftyError {
                    file_name: file_path_name.to_owned(),
                    err,
                })
            }
        },
        Err(err) => Err(PiggytagError::Io(err)),
    }
}

/** Print information */
pub fn prnt_info<S: AsRef<str>>(string: S) {
    println!("{} {}", "[info]".bold(), string.as_ref())
}

/** Separator */
pub fn prnt_sep() {
    println!("{}\n", DEFAULT_SEPARATOR.bold());
}
