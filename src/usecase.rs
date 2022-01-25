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
    fn lookup(&mut self, parent: u64, name: &OsStr) -> Option<attr::Attr>;
    fn attr_from_ino(&self, ino: u64) -> Option<&attr::Attr>;
    fn readdir(&mut self, ino: u64) -> Option<Vec<(u64, &str, attr::FileType)>>;
    fn read(&mut self, ino: u64, offset: i64, size: u64) -> Option<&str>;
    fn write(&mut self, ino: u64, offset: u64, data: &str) -> Result<u64>;
    fn setattr(
        &mut self,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<attr::SystemTime>,
        mtime: Option<attr::SystemTime>
    ) -> Result<attr::Attr>;
    fn create(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32
    ) -> Result<attr::Attr>;
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

    fn lookup(&mut self, parent: u64, name: &OsStr) -> Option<attr::Attr> {
        let attr;
        let entity = match &mut self.entity {
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
                attr = child_attr.clone();
                entity.update_lookupcount(child_ino);
                return Some(attr);
            }
        }
        None
    }

    fn attr_from_ino(&self, ino: u64) -> Option<&attr::Attr> {
        let entity = match &self.entity {
            Some(entity) => entity,
            None => return None
        };

        return entity.attr(&ino);
    }

    fn readdir(&mut self, ino: u64) -> Option<Vec<(u64, &str, attr::FileType)>> {
        let mut ret_vec = Vec::new();
        let entity = match &mut self.entity {
            Some(entity) => entity,
            None => return None
        };
        let st = attr::SystemTime::now();
        entity.update_atime(ino, st);


        let entries = match entity.entry(&ino) {
            Some(entries) => entries,
            None => return None
        };
        let attr = match entity.attr(&ino) {
            Some(attr) => attr,
            None => return None
        };
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
            attr.ctime(),
            attr.nlink()
        );
        self.file_repository.update_attr(&new_attr);

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
            attr.ctime(),
            attr.nlink()
        );
        self.file_repository.update_attr(&new_attr);
        entity.update_atime(ino, st);

        let data = match entity.data(&ino) {
            Some(data) => data,
            None => return None
        };
        let text_data = data.data();
        let end = offset as u64 + size;

        // TODO: offsetを考慮して返却する必要がある
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

        self.file_repository.write_data(ino, new_text_data.as_str())?;
        
        let st = attr::SystemTime::now();
        let data = data::new(ino, new_text_data);

        entity.update_data(ino, data)?;
        entity.update_size(ino, len)?;
        entity.update_mtime(ino, st)?;
        entity.update_ctime(ino, st)?;

        let attr = match entity.attr(&ino) {
            Some(attr) => attr,
            None => return Err(entity::Error::InternalError.into())
        };
        self.file_repository.update_attr(&attr)?;

        return Ok(len);
    }

    fn setattr(
        &mut self,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<attr::SystemTime>,
        mtime: Option<attr::SystemTime>
    ) -> Result<attr::Attr> {
        let imu_entity = match &self.entity {
            Some(entity) => entity,
            None => return Err(entity::Error::InternalError.into()) 
        };
        let mut new_data = String::new();
        if let Some(n) = size {
            match imu_entity.cmp_data_size(ino, n) {
                Ok(c) =>  {
                    match c {
                        entity::Compare::Smaller => {
                            // ずるしてます
                            new_data = smaller_data(imu_entity.data(&ino).unwrap().data(), n);
                        },
                        entity::Compare::Begger => {
                            // ずるしてます
                            new_data = begger_data(imu_entity.data(&ino).unwrap().data(), n);
                        },
                        entity::Compare::Equal => {
                            // 無駄な処理
                            // ずるしてます
                            new_data = imu_entity.data(&ino).unwrap().data().to_string();
                        }
                    }
                },
                Err(_) => return Err(entity::Error::InternalError.into())
            }
        }

        let entity = match &mut self.entity {
            Some(entity) => entity,
            None => return Err(entity::Error::InternalError.into()) 
        };

        if let Some(n) = mode { entity.update_perm(ino, n as u16)?; };
        if let Some(n) = uid { entity.update_uid(ino, n)?; };
        if let Some(n) = gid { entity.update_gid(ino, n)?; };
        if let Some(n) = size {
            entity.update_data(ino, data::new(ino, new_data))?;
            entity.update_size(ino, n)?;
        };
        if let Some(n) = atime { entity.update_atime(ino, n)?; };
        if let Some(n) = mtime { entity.update_mtime(ino, n)?; };

        let attr = match entity.attr(&ino) {
            Some(attr) => attr,
            None => return Err(entity::Error::InternalError.into())
        };
        self.file_repository.update_attr(attr)?;
        // ずるしてます
        self.file_repository.write_data(ino, entity.data(&ino).unwrap().data())?;
        
        return Ok(attr.clone());
    }

    fn create(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32
    ) -> Result<attr::Attr> {
        let entity = match &mut self.entity {
            Some(entity) => entity,
            None => return Err(entity::Error::InternalError.into())
        };
        let name_string = match name.to_str() {
            Some(name) => name.to_string(),
            None => return Err(entity::Error::InternalError.into())
        };
        let ino = entity.new_ino();
        let attr = attr::new(
            ino,
            0,
            name_string,
            attr::FileType::TextFile,
            mode as u16,
            // TODO::ユーザID グループID固定値
            1000,
            1000,
            attr::SystemTime::now(),
            attr::SystemTime::now(),
            attr::SystemTime::now(),
            1
        );

        entity.inc_size(parent)?;
        entity.update_attr(attr.clone());
        entity.update_data(ino, data::new(ino, "".to_string()))?;
        entity.insert_child_ino(parent, ino);

        self.file_repository.update_attr(entity.attr(&parent).unwrap())?;
        self.file_repository.update_attr(entity.attr(&ino).unwrap())?;
        self.file_repository.write_data(ino, "")?;
        self.file_repository.update_entry(ino, entity.entry(&parent).unwrap())?;

        Ok(attr)
    }
}


// この関数テストコード欲しい
// バグが多い気がする
fn merge_str(offset: u64, data: &str, old_str: &str) -> Result<String> {
    let mut new_string_len = offset + data.len() as u64;
    if new_string_len < old_str.len() as u64 {
        new_string_len = old_str.len() as u64;
    }
    let mut new_string_vec = Vec::with_capacity(new_string_len as usize);

    let data_bytes = data.as_bytes();
    let old_str_bytes = old_str.as_bytes();

    for _ in 0..new_string_len {
        new_string_vec.push(0);
    }

    
    for (i, &data) in old_str_bytes.iter().enumerate() {
        new_string_vec[i] = data;
    }

    for (i, &data) in data_bytes.iter().enumerate() {
        new_string_vec[i + offset as usize] = data;
    }

    match String::from_utf8(new_string_vec) {
        Ok(string) => Ok(string),
        Err(e) => Err(e.into())
    }
}

fn begger_data(data: &str, size: u64) -> String {
    let mut new_string_vec = Vec::with_capacity(size as usize);
    let data_bytes = data.as_bytes();

    for &data in data_bytes.iter() {
        new_string_vec.push(data);
    }
    for _ in data.len()..(size as usize) {
        new_string_vec.push(0);
    }

    match String::from_utf8(new_string_vec) {
        Ok(string) => string,
        Err(e) => String::new()
    }
}

fn smaller_data(data: &str,  size: u64) -> String {

    let mut new_string_vec = Vec::with_capacity(size as usize);
    let data_bytes = data.as_bytes();

    for (i, &data) in data_bytes.iter().enumerate() {
        if size  == i as u64 {
            break;
        }
        new_string_vec.push(data);
    }

    match String::from_utf8(new_string_vec) {
        Ok(string) => string,
        Err(e) => String::new()
    }
}