use std::path;
use crate::usecase::repository::File;
use crate::entity::{self, attr, entry, data};
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
    fn init(&mut self, path: &path::Path) -> Result<(u64, attr::AttrsStruct, entry::EntriesStruct, data::AllDataStruct)> {
        self.file_worker.init(path)
    }

    fn write_data(&self, ino: u64, data: &str) -> Result<()> {
        self.file_worker.write_data(ino, data)
    }

    fn update_attr(&self, attr: &attr::Attr) -> Result<()> {
        self.file_worker.update_attr(attr)
    }

    fn update_entry(&self, ino: u64, child_inos: &Vec<entry::Entry>) -> Result<()> {
        self.file_worker.update_entry(ino, child_inos)
    }

    fn del_attr(&self, ino: u64) -> Result<()> {
        self.file_worker.del_attr(ino)
    }

    fn del_data(&self, ino: u64) -> Result<()> {
        self.file_worker.del_data(ino)
    }
}
