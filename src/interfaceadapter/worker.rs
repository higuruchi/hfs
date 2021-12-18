use std::path;
use crate::interfaceadapter::model::{self, attr, data, entry};

pub trait File {
    fn init(&mut self, path: &path::Path) -> Result<model::FileStruct, ()>;
    fn attr_from_ino(&self, path: &path::Path, ino: i64) -> Result<attr::Attr, ()>;
    fn data_from_ino(&self, path: &path::Path, ino: i64) -> Result<data::Data, ()>;
    fn entry_from_ino(&self, path: &path::Path, ino: i64) -> Result<entry::Entry, ()>;
}