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
    InvalidEntry,
    InternalError,
    InvalidAtime
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
            Self::InvalidAtime => write!(f, "There is no atime or atime is invalid")
        } 
    }
}

impl error::Error for Error {}

pub enum Compare {
    Equal,
    Begger,
    Smaller
}

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

    pub fn update_data(&mut self, ino: u64, data: data::Data) -> Result<(), Error> {
        self.data.insert(ino, data);
        return Ok(());
    }

    pub fn update_perm(&mut self, ino: u64, perm: u16) -> Result<(), Error> {
        let attr = match self.attr.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };

        let perm_p = attr.perm_mut();
        *perm_p = perm;

        return Ok(());
    }

    pub fn update_uid(&mut self, ino: u64, uid: u32) -> Result<(), Error> {
        let attr = match self.attr.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };

        let uid_p = attr.uid_mut();
        *uid_p = uid;
        return Ok(());
    }

    pub fn update_gid(&mut self, ino: u64, gid: u32) -> Result<(), Error> {
        let attr = match self.attr.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };

        let gid_p = attr.gid_mut();
        *gid_p = gid;
        return Ok(());
    }

    pub fn update_size(&mut self, ino: u64, size: u64) -> Result<(), Error> {
        let attr = match self.attr.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };
        let size_p = attr.size_mut();
        *size_p = size;
        
        return Ok(());
    }

    pub fn update_atime(&mut self, ino: u64, st: attr::SystemTime) -> Result<(), Error> {
        let attr = match self.attr.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };
        let atime = attr.atime_mut();
        *atime = st;

        return Ok(());
    }

    pub fn update_mtime(&mut self, ino: u64, st: attr::SystemTime) -> Result<(), Error> {
        let attr = match self.attr.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };
        let mtime = attr.mtime_mut();
        *mtime = st;

        return Ok(());
    }

    pub fn update_ctime(&mut self, ino: u64, st: attr::SystemTime) -> Result<(), Error> {
        let attr = match self.attr.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };
        let ctime = attr.ctime_mut();
        *ctime = st;

        return Ok(());
    }

    // ino > size -> Begger
    // ino == size -> Equal
    // ino < size -> Smaller
    pub fn cmp_data_size(&self, ino: u64, size: u64) -> Result<Compare, Error> {
        let attr = match self.attr.get(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError)
        };

        let ino_size = attr.size();

        if ino_size > size {
            return Ok(Compare::Begger);
        }
        if ino_size == size {
            return Ok(Compare::Equal);
        }

        Ok(Compare::Smaller)
    }
}
