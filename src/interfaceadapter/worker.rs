use std::path;
use crate::entity::{self, attr, data, entry};
use anyhow::Result;

pub trait File {
    fn init(&mut self, path: &path::Path) -> Result<entity::FileStruct>;
}
