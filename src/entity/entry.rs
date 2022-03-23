use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Entry {
    // pub ino: u64,
    pub child_ino: u64
}

#[derive(Debug)]
pub struct EntriesStruct {
    entries: HashMap<u64, Vec<Entry>>,
}
pub trait Entries {}

impl Entry {
    pub fn new(
        // ino: u64,
        child_ino: u64
    ) -> Entry {
        Entry {
            // ino: ino,
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

        entry.push(Entry::new(child_ino));

        Some(entry)
    }

    pub fn insert_entry(&mut self, ino: u64) {
        self.entries.insert(ino, Vec::new());
    }

    // TODO: 返却値一時的になし
    pub fn remove_child_ino(&mut self, parent_ino: u64, child_ino: u64) {
        let mut new_entry: Vec<Entry> = Vec::new();

        let entry = match self.entries.get(&parent_ino) {
            Some(entry) => entry,
            None => return
        };

        for e in entry {
            if e.child_ino() != child_ino {
                new_entry.push(Entry::new(e.child_ino()));
            }
        }

        self.entries.insert(parent_ino, new_entry);
    }

    pub fn del(&mut self, ino: u64) {
        self.entries.remove(&ino);
    }
    // fn insert_entry(&mut self, ino: u64) -> Option<&mut Vec<Entry>> {
    //     self.entries().insert(ino, Vec::new());
    //     self.entries().get_mut(&ino)
    // }

    pub fn mov(&mut self, ino: u64, parent_ino: u64, new_parent_ino: u64) {
        // parent_inoのエントリからinoをフィルタリングしたものを挿入
        let mut parent_entry_vec = Vec::new();
        for e in self.entries.get(&parent_ino).unwrap().iter() {
            if e.child_ino() != ino {
                parent_entry_vec.push(e.clone());
            }
        }
        self.entries.insert(parent_ino, parent_entry_vec);

        self.insert_child_ino(new_parent_ino, ino);
    }
}
impl Entries for EntriesStruct {}