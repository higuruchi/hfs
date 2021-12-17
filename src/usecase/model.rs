mod attr;
mod data;
mod entry;

use std::collections::HashMap;
use crate::usecase::model::{attr::Attr, data::Data, entry::Entry};

pub struct FileStruct {
    attr: HashMap<i64, Attr>,
    entry: HashMap<i64, Vec<Entry>>,
    data: HashMap<i64, Data>
}