use std::path;
use crate::usecase::model::FileStruct;

pub trait File {
    fn init(&self, path: &path::Path) -> Result<FileStruct, ()>;
}