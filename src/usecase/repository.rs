use std::path;
use crate::entity;

pub trait File {
    fn init(&mut self, path: &path::Path) -> Result<entity::FileStruct, ()>;
}
