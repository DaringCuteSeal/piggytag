use std::{fmt::Display, path::Path};

use colored::Color;
use lofty::{Accessor, Tag, TaggedFile, TaggedFileExt};

use crate::get_formatted_key_val;

pub struct AudioMetadata {
    pub filename: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub track: Option<u32>,
    pub year: Option<u32>,
    pub comment: Option<String>,
    pub genre: Option<String>,
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
    /** Parse `AudioMetadata` from tag number `idx` from given file. Returns `None` if tag from given index is not available. */
    pub fn parse_from_tagged_file<P: AsRef<Path>>(
        filename: P,
        tagged_file: &TaggedFile,
        idx: usize,
    ) -> Option<Self> {
        let file_name_print = filename
            .as_ref()
            .file_name()
            .unwrap()
            .to_str()
            .map(|x| x.to_owned());
        let tags = tagged_file.tags();
        if tags.is_empty() || idx >= tags.len() {
            return None;
        }
        let tag = tags[idx].clone();
        Some(AudioMetadata {
            // We have to clone some tag.method() results because they return Cow<'_, str> and we need String
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

    /** Mutate tag in-place based on information from `self`. If given `tag_idx` is `None`, a new  */
    pub fn mutate_tag(&self, tag: &mut Tag) {
        if let Some(artist) = &self.artist {
            tag.set_artist(artist.clone());
        }

        if let Some(album) = &self.album {
            tag.set_album(album.clone());
        }

        if let Some(title) = &self.title {
            tag.set_title(title.clone());
        }

        if let Some(track) = self.track {
            tag.set_track(track);
        }

        if let Some(year) = self.year {
            tag.set_year(year);
        }

        if let Some(comment) = &self.comment {
            tag.set_comment(comment.clone());
        }

        if let Some(genre) = &self.genre {
            tag.set_genre(genre.clone());
        }
    }
}
