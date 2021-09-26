pub mod jpeg;
pub mod png;
pub mod svg;
pub mod webp;

use crate::error::Error;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Format {
    Jpeg,
    Png,
    Svg,
    Webp,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Format::*;
        match self {
            Jpeg => write!(f, "jpg"),
            Png => write!(f, "png"),
            Svg => write!(f, "svg"),
            Webp => write!(f, "webp"),
        }
    }
}

impl std::str::FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "jpeg" | "jpg" => Ok(Format::Jpeg),
            "png" => Ok(Format::Png),
            "svg" => Ok(Format::Svg),
            "webp" => Ok(Format::Webp),
            _ => Err(Error::new(format!("{} cannot be optimized", s))),
        }
    }
}

impl Format {
    /// Try to detect the format from the given path. Only looks at the file extension.
    pub fn from_path(path: &str) -> Result<Format, Error> {
        use std::str::FromStr;

        Path::new(path)
            .extension()
            .ok_or(Error::new("invalid file".into()))
            .and_then(|x| x.to_str().ok_or(Error::new("invalid extension".into())))
            .and_then(|x| Format::from_str(x))
    }

    /// Try to detect the format from the given bytes.
    /// Only looks at the header of the file and not checks if the file is valid.
    pub fn from_magic_bytes(data: &[u8]) -> Result<Format, Error> {
        match data {
            bytes if bytes.starts_with(b"\x89PNG") => Ok(Format::Png),
            bytes if check_jpeg(bytes) => Ok(Format::Jpeg),
            bytes if check_webp(bytes) => Ok(Format::Webp),
            bytes if check_svg(bytes) => Ok(Format::Svg),
            _ => Err(Error::new("format not found for bytes".into())),
        }
    }
}

fn check_svg(bytes: &[u8]) -> bool {
    // look for the <svg> tag at the start of the file
    bytes.len() > 4 && bytes.split_at(32).0.windows(4).any(|x| x == b"<svg")
}

fn check_jpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(b"\xFF\xD8\xFF\xDB")
    || bytes.starts_with(b"\xFF\xD8\xFF\xEE")
    || bytes.starts_with(b"\xFF\xD8\xFF\xE0\x00\x10JFIF\x00\x01")
    // FF D8 FF E1??Exif 00 00
    || (bytes.starts_with(b"\xFF\xD8\xFF\xE1")  && bytes.split_at(6).1.starts_with(b"Exif\x00\x00"))
}

fn check_webp(bytes: &[u8]) -> bool {
    // RIFF????WEBP
    bytes.starts_with(b"RIFF") && bytes.split_at(8).1.starts_with(b"WEBP")
}
