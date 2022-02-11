pub mod attr;
pub mod data;
pub mod entry;
pub mod lookup_count;

use std::collections::HashMap;
use std::{error, fmt};
use anyhow::Result;

#[derive(Debug)]
pub enum Error {
    InvalidINO,
    InvalidName,
    InvalidFileType,
    InvalidSize,
    InvalidUID,
    InvalidGID,
    InvalidPERM,
    InvalidData,
    InvalidEntry,
    InternalError,
    InvalidAtime,
    InvalidNlink
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt ::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidINO => write!(f, "There is no inode or inode is invalid"),
            Self::InvalidName => write!(f, "There is no name or name is invalid"),
            Self::InvalidFileType => write!(f, "There is no file type or file type is invalid"),
            Self::InvalidSize => write!(f, "There is no size or size is size invalid"),
            Self::InvalidUID =>write!(f, "There is no uid or uid is invalid"),
            Self::InvalidGID => write!(f, "There is no gid or gid is invalid"),
            Self::InvalidPERM => write!(f, "There is no permission or permission is invalid"),
            Self::InvalidData => write!(f, "There is no data or data is invalid"),
            Self::InvalidEntry => write!(f, "There is no entry or entry is invalid"),
            Self::InternalError => write!(f, "Internal Error"),
            Self::InvalidAtime => write!(f, "There is no atime or atime is invalid"),
            Self::InvalidNlink => write!(f, "There is no nlink or nlink is invalid")
        } 
    }
}

impl error::Error for Error {}