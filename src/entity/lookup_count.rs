use std::{error, fmt};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LookupCount {
    count: HashMap<u64, u64>
}

pub enum Error {}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt ::Formatter<'_>) -> fmt::Result {}
// }

impl LookupCount {
    pub fn new() -> LookupCount {
        LookupCount {
            count: HashMap::new()
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
}