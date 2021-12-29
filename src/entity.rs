pub mod attr;
pub mod data;
pub mod entry;

use std::collections::HashMap;
use std::{error, fmt};

#[derive(Debug)]
pub struct FileStruct {
    attr: HashMap<u64, attr::Attr>,
    entry: HashMap<u64, Vec<entry::Entry>>,
    data: HashMap<u64, data::Data>
}

pub fn new(attr: HashMap<u64, attr::Attr>,
            entry: HashMap<u64, Vec<entry::Entry>>,
            data: HashMap<u64, data::Data>
) -> FileStruct {
    FileStruct {
        attr: attr,
        entry: entry,
        data: data
    }
}

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
    InvalidEntry
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
        } 
    }
}

impl error::Error for Error {}


impl FileStruct {
    pub fn attr(&self, ino: &u64) -> Option<&attr::Attr> {
        match self.attr.get(ino) {
            Some(attr) => return Some(attr),
            None => return None
        }
    }

    pub fn entry(&self, ino: &u64) -> Option<&Vec<entry::Entry>> {
        match self.entry.get(ino) {
            Some(entry) => return Some(entry),
            None => return None
        }
    }

    pub fn data(&self, ino: &u64) -> Option<&data::Data> {
        match self.data.get(ino) {
            Some(data) => return Some(data),
            None => return None
        }
    }
}
