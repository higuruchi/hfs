use std::path;
use crate::entity::{self, attr, data, entry};

pub trait File {
    fn init(&mut self, path: &path::Path) -> Result<entity::FileStruct, ()>;
    fn attr_from_ino(&self, path: &path::Path, ino: u64) -> Result<attr::Attr, ()>;
    fn data_from_ino(&self, path: &path::Path, ino: u64) -> Result<data::Data, ()>;
    fn entry_from_ino(&self, path: &path::Path, ino: u64) -> Result<entry::Entry, ()>;
}