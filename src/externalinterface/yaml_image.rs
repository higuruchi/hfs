use std::path;

use crate::interfaceadapter::{
    worker,
    model::attr,
    model::data,
    model::entry
};

struct YAMLImageStruct {}

pub fn new() -> impl worker::File {
    YAMLImageStruct{}
}

impl worker::File for YAMLImageStruct {
    fn attr_from_ino(&self, path: &path::Path, ino: i64) -> Result<attr::Attr, ()> {
        Ok(attr::new(1, 1, String::from("this is name")))
    }

    fn data_from_ino(&self, path: &path::Path, ino: i64) -> Result<data::Data, ()> {
        Ok(data::new(1, String::from("this is data")))
    }

    fn entry_from_ino(&self, path: &path::Path, ino: i64) -> Result<entry::Entry, ()> {
        Ok(entry::new(1, 1, 1))
    }
}