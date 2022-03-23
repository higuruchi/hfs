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
    fn lookup(&mut self, parent: u64, name: &OsStr) -> Option<fuse::FileAttr>;
    fn getattr(&self, ino: u64) -> Option<fuse::FileAttr>;
    fn readdir(&mut self, ino: u64) -> Option<Vec<(u64, &str, fuse::FileType)>>;
    fn read(&mut self, ino: u64, offset: i64, size: u64) -> Option<&[u8]>;
    fn write(&mut self, ino: u64, offset: u64, data: &[u8]) -> Result<u32>;
    fn setattr(
        &mut self,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<time::Timespec>,
        mtime: Option<time::Timespec>
    ) -> Result<fuse::FileAttr>;
    fn create(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32
    ) -> Result<fuse::FileAttr>;
    fn unlink(
        &mut self,
        parent: u64,
        name: &OsStr
    ) -> Result<()>;

    fn forget(
        &mut self,
        ino: u64,
        nlookup: u64
    ) -> Result<()>;

    fn mkdir(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
    ) -> Result<fuse::FileAttr>;

    fn rmdir(
        &mut self,
        parent: u64,
        name: &OsStr,
    ) -> Result<()>;

    fn rename (
        &mut self,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
    ) -> Result<()>;
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

    fn lookup(&mut self, parent: u64, name: &OsStr) -> Option<fuse::FileAttr> {
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
            atime: timespeck(attr.atime()),
            mtime: timespeck(attr.mtime()),
            ctime: timespeck(attr.ctime()),
            crtime: time::now().to_timespec(),
            kind: file_type,
            perm: attr.perm(),
            nlink: attr.nlink(),
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
            atime: timespeck(attr.atime()),
            mtime: timespeck(attr.mtime()),
            ctime: timespeck(attr.ctime()),
            crtime: time::now().to_timespec(),
            kind: file_type,
            perm: attr.perm(),
            nlink: attr.nlink(),
            uid: attr.uid(),
            gid: attr.gid(),
            rdev: 0,
            flags: 0,
        });
    }

    fn readdir(&mut self, ino: u64) -> Option<Vec<(u64, &str, fuse::FileType)>> {
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
    
    fn read(&mut self, ino: u64, offset: i64, size: u64) -> Option<&[u8]> {
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

    fn setattr(
        &mut self,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<time::Timespec>,
        mtime: Option<time::Timespec>
    ) -> Result<fuse::FileAttr> {
        let atime_systime;
        let mtime_systime;

        if let Some(n) = atime {
            atime_systime = Some(attr::SystemTime::new(n.sec as u64, n.nsec as u32));
        } else {
            atime_systime = None;
        }

        if let Some(n) = mtime {
            mtime_systime = Some(attr::SystemTime::new(n.sec as u64, n.nsec as u32));
        } else {
            mtime_systime = None;
        }

        let attr = match self.usecase.setattr(ino, mode, uid, gid, size, atime_systime, mtime_systime) {
            Ok(attr) => attr,
            Err(e) => return Err(e)
        };

        let file_type = match attr.file_type() {
			attr::FileType::Directory => fuse::FileType::Directory,
			attr::FileType::TextFile => fuse::FileType::RegularFile
		};

        return Ok(fuse::FileAttr {
            ino: attr.ino(),
            size: attr.size(),
            blocks: 0,
            atime: timespeck(attr.atime()),
            mtime: timespeck(attr.mtime()),
            ctime: timespeck(attr.ctime()),
            crtime: time::now().to_timespec(),
            kind: file_type,
            perm: attr.perm(),
            nlink: attr.nlink(),
            uid: attr.uid(),
            gid: attr.gid(),
            rdev: 0,
            flags: 0,
        });
    }

    fn create(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32
    ) -> Result<fuse::FileAttr> {
        let attr = match self.usecase.create(parent, name, mode, flags) {
            Ok(attr) => attr,
            Err(e) => return Err(e) 
        };
        let file_type = match attr.file_type() {
			attr::FileType::Directory => fuse::FileType::Directory,
			attr::FileType::TextFile => fuse::FileType::RegularFile
		};

        Ok(fuse::FileAttr{
            ino: attr.ino(),
            size: attr.size(),
            blocks: 0,
            atime: timespeck(attr.atime()),
            mtime: timespeck(attr.mtime()),
            ctime: timespeck(attr.ctime()),
            crtime: time::now().to_timespec(),
            kind: file_type,
            perm: attr.perm(),
            nlink: attr.nlink(),
            uid: attr.uid(),
            gid: attr.gid(),
            rdev: 0,
            flags: 0,
        })
    }

    fn unlink(
        &mut self,
        parent: u64,
        name: &OsStr
    ) -> Result<()> {
        match self.usecase.unlink(parent, name) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn forget(
        &mut self,
        ino: u64,
        nlookup: u64,
    ) -> Result<()> {
        self.usecase.forget(ino, nlookup)
    }

    fn mkdir(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
    ) -> Result<fuse::FileAttr> {
        let attr = self.usecase.mkdir(parent, name, mode)?;
        let file_type = match attr.file_type() {
			attr::FileType::Directory => fuse::FileType::Directory,
			attr::FileType::TextFile => fuse::FileType::RegularFile
		};

        Ok(fuse::FileAttr{
            ino: attr.ino(),
            size: attr.size(),
            blocks: 0,
            atime: timespeck(attr.atime()),
            mtime: timespeck(attr.mtime()),
            ctime: timespeck(attr.ctime()),
            crtime: time::now().to_timespec(),
            kind: file_type,
            perm: attr.perm(),
            nlink: attr.nlink(),
            uid: attr.uid(),
            gid: attr.gid(),
            rdev: 0,
            flags: 0,
        })
    }

    fn rmdir(
        &mut self,
        parent: u64,
        name: &OsStr,
    ) -> Result<()> {
        self.usecase.rmdir(parent, name)
    }

    fn rename (
        &mut self,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
    ) -> Result<()> {
        self.usecase.rename(parent, name, newparent, newname)
    }
}

fn timespeck(st: attr::SystemTime) -> time::Timespec {
    time::Timespec::new(st.as_secs() as i64, st.subsec_nanos() as i32)
}