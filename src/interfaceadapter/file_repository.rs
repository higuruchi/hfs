use std::path;
use crate::usecase::repository::File;
use crate::entity;
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
        let files = match self.file_worker.init(path) {
            Ok(files) => files,
            Err(e) => return Err(e)
        };

        return Ok(files); 
    }
}
