use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Entry {
    pub ino: u64,
    pub child_ino: u64
}

#[derive(Debug)]
pub struct EntriesStruct {
    entries: HashMap<u64, Vec<Entry>>,
}
pub trait Entries {}

impl Entry {
    pub fn new(
        ino: u64,
        child_ino: u64
    ) -> Entry {
        Entry {
            ino: ino,
            child_ino: child_ino
        }
    }

    pub fn child_ino(&self) -> u64 {
        return self.child_ino;
    }
}

impl EntriesStruct {
    pub fn new(entries: HashMap<u64, Vec<Entry>>) -> EntriesStruct {
        EntriesStruct {
            entries: entries
        }
    }

    pub fn entries(&self) -> &HashMap<u64, Vec<Entry>> {
        &self.entries
    }

    pub fn entry(&self, ino: u64) -> Option<&Vec<Entry>> {
        match self.entries.get(&ino) {
            Some(entry) => return Some(entry),
            None => return None
        }
    }

    pub fn insert_child_ino(&mut self, parent_ino: u64, child_ino: u64) -> Option<&Vec<Entry>> {
        let entry = match self.entries.get_mut(&parent_ino) {
            Some(entry) => entry,
            None => return None
        };

        entry.push(Entry::new(parent_ino, child_ino));

        Some(entry)
    }

    // fn insert_entry(&mut self, ino: u64) -> Option<&mut Vec<Entry>> {
    //     self.entries().insert(ino, Vec::new());
    //     self.entries().get_mut(&ino)
    // }
}
impl Entries for EntriesStruct {}