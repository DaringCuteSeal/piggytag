use std::{fs::File, path::Path};

use colored::{Color, Colorize};
use error::PiggytagError;
use lofty::{read_from, Probe, TaggedFile};

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
    let result = match Probe::open(filename.as_ref()) {
        Ok(res) => res,
        Err(err) => return Err(PiggytagError::LoftyError(err)),
    };

    match result.guess_file_type() {
        Ok(result) => match result.read() {
            Ok(file) => Ok(file),
            Err(err) => Err(PiggytagError::LoftyError(err)),
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
