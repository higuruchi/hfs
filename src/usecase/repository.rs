use std::path;
use crate::entity::{self, attr, entry, data};
use anyhow::Result;

pub trait File {
    fn init(&mut self, path: &path::Path) -> Result<(u64, attr::AttrsStruct, entry::EntriesStruct, data::AllDataStruct)>;
    fn write_data(&self, ino: u64, data: &str) -> Result<()>;
    fn update_attr(&self, attr: &attr::Attr) -> Result<()>;
    fn update_entry(&self, ino: u64, child_inos: &Vec<entry::Entry>) -> Result<()>;
}