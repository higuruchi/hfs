use std::path;
use crate::entity::{self, attr, data, entry};

pub trait File {
    fn init(&mut self, path: &path::Path) -> Result<entity::FileStruct, ()>;
}
