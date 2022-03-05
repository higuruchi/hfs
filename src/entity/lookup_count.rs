use std::{error, fmt};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LookupCount {
    count: HashMap<u64, u64>,
    unlink_delay: HashMap<u64, bool>
}

pub enum Error {
    InoNotFound
}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt ::Formatter<'_>) -> fmt::Result {}
// }

impl LookupCount {
    pub fn new() -> LookupCount {
        LookupCount {
            count: HashMap::new(),
            unlink_delay: HashMap::new()
        }
    }

    pub fn update_lookupcount(&mut self, ino: u64) -> Result<(), Error> {
        let lc = self.count.entry(ino).or_insert(0);
        *lc += 1;
        Ok(())
    }

    pub fn lookup_count(&self, ino: u64) -> u64 {
        match self.count.get(&ino) {
            Some(lc) => *lc,
            None => 0
        }
    }

    pub fn delay(&mut self, ino: u64) {
        self.unlink_delay.insert(ino, true);
    }

    pub fn delay_parent(&self, ino: u64) -> Option<bool> {
        match self.unlink_delay.get(&ino) {
            Some(flg) => Some(*flg),
            None => None
        }
    }

    pub fn forget(&mut self, ino: u64, nlookup: u64) -> Option<u64> {
        if let Some(lookup_count) = self.count.get(&ino) {
            let new_lookup_count = lookup_count - nlookup;

            self.count.insert(ino, new_lookup_count)
        } else {
            None
        }

    }
}