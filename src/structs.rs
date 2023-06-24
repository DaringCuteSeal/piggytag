use std::{borrow::Cow, fmt::Display, path::Path};

use colored::Color;
use lofty::{Accessor, Tag, TagExt, TaggedFile, TaggedFileExt};

use crate::get_formatted_key_val;

pub struct AudioMetadata {
    filename: Option<String>,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    track: Option<u32>,
    year: Option<u32>,
    comment: Option<String>,
    genre: Option<String>,
}

impl Display for AudioMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut retval = String::new();
        retval.push_str(&get_formatted_key_val(
            "Filename",
            self.filename.as_deref(),
            Color::Cyan,
        ));
        retval.push_str(&get_formatted_key_val(
            "Title",
            self.title.as_deref(),
            Color::Cyan,
        ));
        retval.push_str(&get_formatted_key_val(
            "Artist",
            self.artist.as_deref(),
            Color::Cyan,
        ));
        retval.push_str(&get_formatted_key_val(
            "Album",
            self.album.as_deref(),
            Color::Magenta,
        ));
        retval.push_str(&get_formatted_key_val("Track", self.track, Color::Yellow));

        retval.push_str(&get_formatted_key_val("Year", self.year, Color::Yellow));

        retval.push_str(&get_formatted_key_val(
            "Comment",
            self.comment.as_deref(),
            Color::Cyan,
        ));

        retval.push_str(&get_formatted_key_val(
            "Genre",
            self.genre.as_deref(),
            Color::Cyan,
        ));
        write!(f, "{}", retval)
    }
}

impl AudioMetadata {
    pub fn parse_from_tagged_file<P: AsRef<Path>>(
        filename: P,
        tagged_file: TaggedFile,
    ) -> Option<Self> {
        let file_name_print = filename
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .map(|x| x.to_owned());
        let tag = match tagged_file.primary_tag() {
            Some(tag) => tag,
            None => return None,
        };
        Some(AudioMetadata {
            // We have to clone some tag.method() results because they return Option<Cow<'_, str>> and we need Option<String>
            filename: file_name_print,
            title: tag.title().map(|x| x.as_ref().to_owned()),
            artist: tag.artist().map(|x| x.as_ref().to_owned()),
            album: tag.album().map(|x| x.as_ref().to_owned()),
            track: tag.track(),
            year: tag.year(),
            comment: tag.comment().map(|x| x.as_ref().to_owned()),
            genre: tag.genre().map(|x| x.as_ref().to_owned()),
        })
    }
}
