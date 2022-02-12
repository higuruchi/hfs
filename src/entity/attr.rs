use std::collections::HashMap;
use std::{error, fmt};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Attr {
    pub ino: u64,
    pub size: u64,
    pub name: String,
    // pub blocks: u32,
    pub atime: SystemTime,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    pub kind: FileType,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    // pub rdev: u32,
    // pub flags: u32,
}

#[derive (Clone, Copy, Debug)]
pub enum FileType {
    Directory,
    TextFile
}

#[derive(Clone, Copy, Debug)]
pub struct SystemTime(pub u64, pub u32);

#[derive(Debug)]
pub struct AttrsStruct {
    attrs: HashMap<u64, Attr>,
}
pub trait Attrs {}

pub enum Error {
    InternalError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt ::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal Error"),
        } 
    }
}

pub enum Compare {
    Equal,
    Begger,
    Smaller
}

impl Attr {
    pub fn new(
        ino: u64,
        size: u64,
        name: String,
        kind: FileType,
        perm: u16,
        uid: u32,
        gid: u32,
        atime: SystemTime,
        mtime: SystemTime,
        ctime: SystemTime,
        nlink: u32
    ) -> Attr {
        Attr {
            ino: ino,
            size: size,
            name: name,
            kind: kind,
            perm: perm,
            uid: uid,
            gid: gid,
            atime: atime,
            mtime: mtime,
            ctime: ctime,
            nlink: nlink
        }
    }

    pub fn ino(&self) -> u64 {
        self.ino
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_type(&self) -> FileType {
        self.kind
    }

    pub fn perm(&self) -> u16 {
        self.perm
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }

    pub fn kind(&self) -> FileType {
        self.kind
    }

    pub fn atime(&self) -> SystemTime {
        self.atime
    }

    pub fn mtime(&self) -> SystemTime {
        self.mtime
    }

    pub fn ctime(&self) -> SystemTime {
        self.ctime
    }

    pub fn nlink(&self) -> u32 {
        self.nlink
    }

    pub fn perm_mut(&mut self) -> &mut u16 {
        &mut self.perm
    }

    pub fn uid_mut(&mut self) -> &mut u32 {
        &mut self.uid
    }

    pub fn gid_mut(&mut self) -> &mut u32 {
        &mut self.gid
    }

    pub fn size_mut(&mut self) -> &mut u64 {
        &mut self.size
    }

    pub fn atime_mut(&mut self) -> &mut SystemTime {
        &mut self.atime
    }

    pub fn mtime_mut(&mut self) -> &mut SystemTime {
        &mut self.mtime
    }

    pub fn ctime_mut(&mut self) -> &mut SystemTime {
        &mut self.ctime
    }

    pub fn nlink_mut(&mut self) -> &mut u32 {
        &mut self.nlink
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        let now = std::time::SystemTime::now();
        if let Ok(epoch) = now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            SystemTime(epoch.as_secs(), epoch.subsec_nanos())
        } else {
            SystemTime(0, 0)
        }
    }

    pub fn new(sec: u64, nsec: u32) -> SystemTime {
        SystemTime(sec, nsec)
    }

    pub fn as_secs(&self) -> u64 {
        return self.0
    }

    pub fn subsec_nanos(&self) -> u32 {
        return self.1
    }
}

impl AttrsStruct {
    pub fn new(attrs: HashMap<u64, Attr>) -> AttrsStruct {
        AttrsStruct{
            attrs: attrs,
        }
    }

    pub fn attr(&self, ino: u64) -> Option<&Attr> {
        match self.attrs.get(&ino) {
            Some(attr) => Some(attr),
            None => None
        }
    }

    pub fn update_atime(&mut self, ino: u64, st: SystemTime) -> Result<(), Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };
        let atime = attr.atime_mut();
        *atime = st;

        return Ok(());
    }

    pub fn update_size(&mut self, ino: u64, size: u64) -> Result<(), Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };
        let size_p = attr.size_mut();
        *size_p = size;
        
        return Ok(());
    }

    pub fn update_mtime(&mut self, ino: u64, st: SystemTime) -> Result<(), Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };
        let mtime = attr.mtime_mut();
        *mtime = st;

        return Ok(());
    }

    pub fn update_ctime(&mut self, ino: u64, st: SystemTime) -> Result<(), Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };
        let ctime = attr.ctime_mut();
        *ctime = st;

        return Ok(());
    }

    pub fn inc_size(&mut self, ino: u64) -> Result<u64, Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };

        let size_p = attr.size_mut();
        let size = *size_p + 1;
        *size_p = size;

        Ok(size)
    }

    pub fn dec_size(&mut self, ino: u64) -> Result<u64, Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };

        let size_p = attr.size_mut();
        let size = *size_p - 1;
        *size_p = size;

        Ok(size)
    }

    pub fn update_attr(&mut self, attr: Attr) -> Option<Attr> {
        self.attrs.insert(attr.ino(), attr)
    }

    pub fn update_perm(&mut self, ino: u64, perm: u16) -> Result<(), Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };

        let perm_p = attr.perm_mut();
        *perm_p = perm;

        return Ok(());
    }

    pub fn update_uid(&mut self, ino: u64, uid: u32) -> Result<(), Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };

        let uid_p = attr.uid_mut();
        *uid_p = uid;
        return Ok(());
    }

    pub fn update_gid(&mut self, ino: u64, gid: u32) -> Result<(), Error> {
        let attr = match self.attrs.get_mut(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
        };

        let gid_p = attr.gid_mut();
        *gid_p = gid;
        return Ok(());
    }

    pub fn del(&mut self, ino: u64) -> Result<Attr, Error> {
        match self.attrs.remove(&ino) {
            Some(attr) => Ok(attr),
            None => Err(Error::InternalError.into())
        }
    }

    // ino > size -> Begger
    // ino == size -> Equal
    // ino < size -> Smaller
    pub fn cmp_data_size(&self, ino: u64, size: u64) -> Result<Compare, Error> {
        let attr = match self.attrs.get(&ino) {
            Some(attr) => attr,
            None => return Err(Error::InternalError.into())
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

impl Attrs for AttrsStruct {}
