use std::path;
use crate::usecase::repository::File;
use crate::usecase::model::{self, FileStruct};

struct FileRepositoryStruct {}

pub fn new() -> impl File {
    FileRepositoryStruct{}
}

impl File for FileRepositoryStruct {
    fn init(&self, path: &path::Path) -> Result<FileStruct, ()> {
        println!("called file_repository init");
        return Err(()); 
    }
}