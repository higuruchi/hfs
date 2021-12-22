use crate::usecase;
use std::path::Path;
use std::ffi::OsStr;
use crate::entity::attr;
use fuse;
use time;

struct ControllerStruct<U: usecase::Usecase> {
    usecase: U
}

pub trait Controller {
    fn init(&mut self, config: &String) -> Result<(), ()>;
    fn lookup(&self, parent: u64, name: &OsStr) -> Option<fuse::FileAttr>;
    fn getattr(&self, ino: u64) -> Option<fuse::FileAttr>;
}

pub fn new<U>(usecase: U) -> impl Controller
    where U: usecase::Usecase
{
    ControllerStruct{
        usecase: usecase
    }
}

impl<U: usecase::Usecase> Controller for ControllerStruct<U> {
    fn init(&mut self, config: &String) -> Result<(), ()> {
        self.usecase.init(&Path::new(config));
        return Ok(());
    }

    fn lookup(&self, parent: u64, name: &OsStr) -> Option<fuse::FileAttr> {
        let attr = match self.usecase.lookup(parent, name) {
            Some(attr) => attr,
            None => return None
        };

        return Some(fuse::FileAttr {
            ino: attr.ino(),
            size: attr.size(),
            blocks: 0,
            atime: time::Timespec{sec: 1, nsec: 0},
            mtime: time::Timespec{sec: 1, nsec: 0},
            ctime: time::Timespec{sec: 1, nsec: 0},
            crtime: time::Timespec{sec: 1, nsec: 0},
            kind: fuse::FileType::RegularFile,
            perm: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        });
    }

    fn getattr(&self, ino: u64) -> Option<fuse::FileAttr> {
        let attr = match self.usecase.attr_from_ino(ino) {
            Some(attr) => attr,
            None => return None
        };

        return Some(fuse::FileAttr {
            ino: attr.ino(),
            size: attr.size(),
            blocks: 0,
            atime: time::Timespec{sec: 1, nsec: 0},
            mtime: time::Timespec{sec: 1, nsec: 0},
            ctime: time::Timespec{sec: 1, nsec: 0},
            crtime: time::Timespec{sec: 1, nsec: 0},
            kind: fuse::FileType::RegularFile,
            perm: 0,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        });
    }
}