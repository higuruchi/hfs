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