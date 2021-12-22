pub mod repository;

use std::path;
use std::ffi::OsStr;
use crate::entity::{self, attr};

#[derive(Debug)]
struct UsecaseStruct<F: repository::File> {
    entity: Option<entity::FileStruct>,
    file_repository: F
}

pub trait Usecase {
    fn init(&mut self, path: &path::Path) -> Result<(), ()>;
    fn lookup(&self, parent: u64, name: &OsStr) -> Option<&attr::Attr>;
    fn attr_from_ino(&self, ino: u64) -> Option<&attr::Attr>;
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
    fn init(&mut self, path: &path::Path) -> Result<(), ()> {
        match self.file_repository.init(path) {
            Ok(file_struct) => {
                self.entity = Some(file_struct);
                return Ok(());
            },
            Err(_) => return Err(())
        };
    }

    fn lookup(&self, parent: u64, name: &OsStr) -> Option<&attr::Attr> {
        let entity = match &self.entity {
            Some(entity) => entity,
            None => return None
        };
        let entries = match entity.entry(&(parent as i64)) {
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

        return entity.attr(&(ino as i64));
    }
}