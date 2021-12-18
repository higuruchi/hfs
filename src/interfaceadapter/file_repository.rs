use std::path;
use crate::usecase::repository::File;
use crate::usecase::model::{self, FileStruct};
use crate::interfaceadapter::{worker};

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
    fn init(&mut self, path: &path::Path) -> Result<FileStruct, ()> {
        let attr = self.file_worker.attr_from_ino(path, 1);
        let data = self.file_worker.data_from_ino(path, 1);
        let entry = self.file_worker.entry_from_ino(path, 1);
        let files = self.file_worker.init(path);
        return Err(()); 
    }
}