pub mod repository;

use std::path;
use std::ffi::OsStr;
use anyhow::Result;
use crate::entity::{self, attr, data};

#[derive(Debug)]
struct UsecaseStruct<F: repository::File> {
    entity: Option<entity::FileStruct>,
    file_repository: F
}

pub trait Usecase {
    fn init(&mut self, path: &path::Path) -> Result<()>;
    fn lookup(&self, parent: u64, name: &OsStr) -> Option<&attr::Attr>;
    fn attr_from_ino(&self, ino: u64) -> Option<&attr::Attr>;
    fn readdir(&self, ino: u64) -> Option<Vec<(u64, &str, attr::FileType)>>;
    fn read(&mut self, ino: u64, offset: i64, size: u64) -> Option<&str>;
    fn write(&mut self, ino: u64, offset: u64, data: &str) -> Result<u64>;
}

pub fn new<F>(file_repository: F) -> impl Usecase 
    where F: repository::File
{
    UsecaseStruct{
        entity: None,
        file_repository: file_repository
    }
}

impl<F: repository::File> Usecase for UsecaseStruct<F> {
    fn init(&mut self, path: &path::Path) -> Result<()> {
       match self.file_repository.init(path) {
            Ok(file_struct) => {
                self.entity = Some(file_struct);
                return Ok(());
            },
            Err(e) => return Err(e)
        };
    }

    fn lookup(&self, parent: u64, name: &OsStr) -> Option<&attr::Attr> {
        let entity = match &self.entity {
            Some(entity) => entity,
            None => return None
        };
        let entries = match entity.entry(&parent) {
            Some(entries) => entries,
            None => return None
        };

        for entry in entries.iter() {
            let child_ino = entry.child_ino();
            let child_attr = match entity.attr(&child_ino) {
                Some(child_attr) => child_attr,
                None => return None
            };
            let file_name = match name.to_str() {
                Some(file_name) => file_name,
                None => return None
            };

            if child_attr.name == file_name {
                return Some(child_attr);
            }
        }

        return None;
    }

    fn attr_from_ino(&self, ino: u64) -> Option<&attr::Attr> {
        let entity = match &self.entity {
            Some(entity) => entity,
            None => return None
        };

        return entity.attr(&ino);
    }

    fn readdir(&self, ino: u64) -> Option<Vec<(u64, &str, attr::FileType)>> {
        let mut ret_vec = Vec::new();
        let entity = match &self.entity {
            Some(entity) => entity,
            None => return None
        };
        let entries = match entity.entry(&ino) {
            Some(entries) => entries,
            None => return None
        };

        for entry in entries.iter() {
            let child_ino = entry.child_ino();
            let child_attr = match entity.attr(&child_ino) {
                Some(child_attr) => child_attr,
                None => return None
            };
            let file_name = child_attr.name();
            let file_type = child_attr.file_type();

            ret_vec.push((child_ino, file_name, file_type));
        }

        return Some(ret_vec);
    }
    
    fn read(&mut self, ino: u64, offset: i64, size: u64) -> Option<&str> {
        let entity = match &mut self.entity {
            Some(entity) => entity,
            None => return None
        };
        let attr = match entity.attr(&ino) {
            Some(attr) => attr,
            None => return None
        };
        let st = attr::SystemTime::now();
        let new_attr = attr::new(
            ino,
            attr.size(),
            attr.name().to_string(),
            attr.kind(),
            attr.perm(),
            attr.uid(),
            attr.gid(),
            st,
            attr.mtime(),
            attr.ctime()
        );
        self.file_repository.update_attr(&new_attr);
        entity.update_atime(ino, st);

        let data = match entity.data(&ino) {
            Some(data) => data,
            None => return None
        };
        let text_data = data.data();
        let end = offset as u64 + size;

        // offsetを考慮して返却する必要がある
        return Some(text_data);
//        return Some(&text_data[(offset as usize)..(end as usize)]);
    }

    fn write(&mut self, ino: u64, offset: u64, data: &str) -> Result<u64> {
        let entity = match &mut self.entity {
            Some(entity) => entity,
            None => return Err(entity::Error::InternalError.into()) 
        }; 
        let old_data = match entity.data(&ino) {
            Some(data) => data,
            None => return Err(entity::Error::InternalError.into())
        };
        let old_str = old_data.data();
        let new_text_data = match merge_str(offset, data, old_str) {
            Ok(new_text_data) => new_text_data,
            Err(e) => return Err(e.into())
        };
        let len: u64 = new_text_data.len() as u64;

        self.file_repository.write_data(ino, new_text_data.as_str());
        
        let attr = match entity.attr(&ino) {
            Some(attr) => attr,
            None => return Err(entity::Error::InternalError.into())
        };
        let st = attr::SystemTime::now();
        let new_attr = attr::new(
            ino,
            len,
            attr.name().to_string(),
            attr.kind(),
            attr.perm(),
            attr.uid(),
            attr.gid(),
            attr.atime(),
            st,
            st
        );
        self.file_repository.update_attr(&new_attr);

        let data = data::new(ino, new_text_data);

        entity.update_data(ino, data);
        entity.update_size(ino, len);
        return Ok(len);
    }
}

fn merge_str(offset: u64, data: &str, old_str: &str) -> Result<String> {
    let mut new_string_len = offset + data.len() as u64;
    if new_string_len < old_str.len() as u64 {
        new_string_len = old_str.len() as u64;
    }
    let mut new_string_vec = Vec::with_capacity(new_string_len as usize);

    let data_bytes = data.as_bytes();
    let old_str_bytes = old_str.as_bytes();

    for &data in old_str_bytes.iter() {
        new_string_vec.push(data);
    }

    // ----------------------
    let data_end_offset = offset + data.len() as u64;
    let old_str_len = old_str.len() as u64;

    if 0 <= data_end_offset && data_end_offset < old_str_len {
        for (i, &data) in data_bytes.iter().enumerate() {
            new_string_vec[i + offset as usize] = data;
        }
    }

    if offset < old_str_len && old_str_len < data_end_offset {
        for (i, &data) in data_bytes.iter().enumerate() {
            if (old_str.len() as u64) < i as u64 {
                new_string_vec.push(data);
                continue;
            }
            new_string_vec[i + offset as usize] = data;
        }
    }

    if old_str_len <= offset {
        for &data in data_bytes.iter() {
            new_string_vec.push(data);
        }
    }
    // ----------------------

    match String::from_utf8(new_string_vec) {
        Ok(string) => Ok(string),
        Err(e) => Err(e.into())
    }
}
