use crate::fs;

#[derive(Debug)]
pub struct Entry {
    pub ino: i64,
    pub parent_ino: i64,
    pub child_ino: i64,
    pub filename: String,
    pub file_type: fs::FileType
}

pub fn new(
    ino: i64,
    parent_ino: i64,
    child_ino: i64,
    filename: String,
    file_type: fs::FileType
) -> Entry {
    Entry {
        ino: ino,
        parent_ino: parent_ino,
        child_ino: child_ino,
        filename: filename,
        file_type: file_type
    }
}