pub mod attr;
pub mod data;
pub mod entry;

use std::collections::HashMap;

#[derive(Debug)]
pub struct FileStruct {
    attr: HashMap<i64, attr::Attr>,
    entry: HashMap<i64, Vec<entry::Entry>>,
    data: HashMap<i64, data::Data>
}

pub fn new(attr: HashMap<i64, attr::Attr>,
            entry: HashMap<i64, Vec<entry::Entry>>,
            data: HashMap<i64, data::Data>
) -> FileStruct {
    FileStruct {
        attr: attr,
        entry: entry,
        data: data
    }
}

impl FileStruct {
    pub fn attr(&self, ino: &i64) -> Option<&attr::Attr> {
        match self.attr.get(ino) {
            Some(attr) => return Some(attr),
            None => return None
        }
    }

    pub fn entry(&self, ino: &i64) -> Option<&Vec<entry::Entry>> {
        match self.entry.get(ino) {
            Some(entry) => return Some(entry),
            None => return None
        }
    }
}