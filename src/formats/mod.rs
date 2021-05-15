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
    pub fn from_path(path: &str) -> Result<Format, Error> {
        use std::str::FromStr;

        Path::new(path)
            .extension()
            .ok_or(Error::new("invalid file".into()))
            .and_then(|x| x.to_str().ok_or(Error::new("invalid extension".into())))
            .and_then(|x| Format::from_str(x))
    }
}
