use crate::fs;

#[derive(Debug, Clone)]
pub struct Entry {
    pub ino: i64,
    pub parent_ino: i64,
    pub child_ino: i64
}

pub fn new(
    ino: i64,
    parent_ino: i64,
    child_ino: i64
) -> Entry {
    Entry {
        ino: ino,
        parent_ino: parent_ino,
        child_ino: child_ino
    }
}