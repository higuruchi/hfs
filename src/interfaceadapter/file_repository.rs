use std::path;
use crate::usecase::repository::File;
use crate::entity::{self, attr};
use crate::interfaceadapter::{worker};
use anyhow::Result;

struct FileRepositoryStruct<F: worker::File> {
    file_worker: F
}

pub fn new<F>(file_worker: F) -> impl File 
    where F: worker::File
{
    FileRepositoryStruct{
        file_worker: file_worker
    }
}

impl<F: worker::File> File for FileRepositoryStruct<F> {
    fn init(&mut self, path: &path::Path) -> Result<entity::FileStruct> {
        match self.file_worker.init(path) {
            Ok(files) => Ok(files),
            Err(e) => Err(e)
        }
    }

    fn write_data(&self, ino: u64, data: &str) -> Result<()> {
        match self.file_worker.write_data(ino, data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }

    fn update_attr(&self, attr: &attr::Attr) -> Result<()> {
        match self.file_worker.update_attr(attr) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }
}
