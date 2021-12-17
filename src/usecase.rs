pub mod repository;
pub mod model;

use std::path;

struct UsecaseStruct<F: repository::File> {
    file_repository: F
}

pub trait Usecase {
    fn init(&self, path: &path::Path) -> Result<(), ()>;
}

pub fn new<F>(file_repository: F) -> impl Usecase 
    where F: repository::File
{
    UsecaseStruct{
        file_repository: file_repository
    }
}

impl<F: repository::File> Usecase for UsecaseStruct<F> {
    fn init(&self, path: &path::Path) -> Result<(), ()> {
        let file_struct = self.file_repository.init(path);
        return Ok(());
    }
}