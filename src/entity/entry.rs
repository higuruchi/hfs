#[derive(Debug, Clone)]
pub struct Entry {
    pub ino: u64,
    pub child_ino: u64
}

pub fn new(
    ino: u64,
    child_ino: u64
) -> Entry {
    Entry {
        ino: ino,
        child_ino: child_ino
    }
}

impl Entry {
    pub fn child_ino(&self) -> u64 {
        return self.child_ino;
    }
}