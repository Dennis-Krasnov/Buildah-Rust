use std::{io, string};

use crate::buildah_error::BuildahError::{InvalidUTF8Output, IoError};

/// Custom top-level error.
#[derive(Debug)]
pub enum BuildahError {
    Unknown(String),
    InvalidImage(String),
    Buildah(String),
    InvalidUTF8Output(string::FromUtf8Error),
    IoError(io::Error),
}

impl From<string::FromUtf8Error> for BuildahError {
    fn from(error: string::FromUtf8Error) -> Self {
        InvalidUTF8Output(error)
    }
}

impl From<io::Error> for BuildahError {
    fn from(error: io::Error) -> Self {
        IoError(error)
    }
}
