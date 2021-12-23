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
    fn readdir(&self, ino: u64) -> Option<Vec<(u64, &str, fuse::FileType)>>;
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

    fn readdir(&self, ino: u64) -> Option<Vec<(u64, &str, fuse::FileType)>> {
        let mut return_vec = Vec::new();
        let files_data = match self.usecase.readdir(ino) {
            Some(files_data) => files_data,
            None => return None
        };

        for file_data in files_data.iter() {
            let file_type = match file_data.2 {
                attr::FileType::Directory => fuse::FileType::Directory,
                attr::FileType::TextFile => fuse::FileType::RegularFile
            };
            return_vec.push((file_data.0, file_data.1, file_type));
        }

        return Some(return_vec);
    }
}