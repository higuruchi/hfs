use crate::usecase;
use std::path::Path;
use std::ffi::OsStr;
use crate::entity::attr;
use anyhow::Result;
use fuse;
use time;
use std::str;

struct ControllerStruct<U: usecase::Usecase> {
    usecase: U
}

pub trait Controller {
    fn init(&mut self, config: &String) -> Result<()>;
    fn lookup(&self, parent: u64, name: &OsStr) -> Option<fuse::FileAttr>;
    fn getattr(&self, ino: u64) -> Option<fuse::FileAttr>;
    fn readdir(&self, ino: u64) -> Option<Vec<(u64, &str, fuse::FileType)>>;
    fn read(&self, ino: u64, offset: i64, size: u64) -> Option<&[u8]>;
    fn write(&mut self, ino: u64, offset: u64, data: &[u8]) -> Result<u32>;
}

pub fn new<U>(usecase: U) -> impl Controller
    where U: usecase::Usecase
{
    ControllerStruct{
        usecase: usecase
    }
}

impl<U: usecase::Usecase> Controller for ControllerStruct<U> {
    fn init(&mut self, config: &String) -> Result<()> {
		match self.usecase.init(&Path::new(config)) {
			Ok(_) => Ok(()),
			Err(e) => return Err(e)
		}
    }

    fn lookup(&self, parent: u64, name: &OsStr) -> Option<fuse::FileAttr> {
        let attr = match self.usecase.lookup(parent, name) {
            Some(attr) => attr,
            None => return None
        };
		let file_type = match attr.file_type() {
			attr::FileType::Directory => fuse::FileType::Directory,
			attr::FileType::TextFile => fuse::FileType::RegularFile
		};

        return Some(fuse::FileAttr {
            ino: attr.ino(),
            size: attr.size(),
            blocks: 0,
            atime: time::now().to_timespec(),
            mtime: time::now().to_timespec(),
            ctime: time::now().to_timespec(),
            crtime: time::now().to_timespec(),
            kind: file_type,
            perm: attr.perm(),
            nlink: 2,
            uid: attr.uid(),
            gid: attr.gid(),
            rdev: 0,
            flags: 0,
        });
    }

    fn getattr(&self, ino: u64) -> Option<fuse::FileAttr> {
        let attr = match self.usecase.attr_from_ino(ino) {
            Some(attr) => attr,
            None => return None
        };
		let file_type = match attr.file_type() {
			attr::FileType::Directory => fuse::FileType::Directory,
			attr::FileType::TextFile => fuse::FileType::RegularFile
		};
		

        return Some(fuse::FileAttr {
            ino: attr.ino(),
            size: attr.size(),
            blocks: 0,
            atime: time::now().to_timespec(),
            mtime: time::now().to_timespec(),
            ctime: time::now().to_timespec(),
            crtime: time::now().to_timespec(),
            kind: file_type,
            perm: attr.perm(),
            nlink: 2,
            uid: attr.uid(),
            gid: attr.gid(),
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
    
    fn read(&self, ino: u64, offset: i64, size: u64) -> Option<&[u8]> {
        let mut ret_data: Vec<u8> = Vec::with_capacity(size as usize);
        let data = match self.usecase.read(ino, offset, size) {
            Some(data) => data,
            None => return None
        };
        let ret_data = data.as_bytes();

        return Some(ret_data);
    }

    fn write(&mut self, ino: u64, offset: u64, data: &[u8]) -> Result<u32>{
        let data_str = str::from_utf8(data).unwrap();

        let size = match self.usecase.write(ino, offset, data_str) { 
            Ok(size) => size,
            Err(e) => return Err(e)
        };
        return Ok(size as u32); 
    }
}
