use std::path;
use crate::entity::{self, attr, data, entry};
use anyhow::Result;

pub trait File {
    fn init(&mut self, path: &path::Path) -> Result<entity::FileStruct>;
    fn write_data(&self, ino: u64, data: &str) -> Result<()>;
    fn update_attr(&self, attr: &attr::Attr) -> Result<()>;
    fn update_entry(&self, ino: u64, child_inos: &Vec<entry::Entry>) -> Result<()>;
}
