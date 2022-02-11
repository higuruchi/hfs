use std::collections::HashMap;

#[derive(Debug)]
pub struct Data {
    pub ino: u64,
    data: String
}

#[derive(Debug)]
pub struct AllDataStruct {
    all_data: HashMap<u64, Data>
}
pub trait AllData {}

impl Data {
    pub fn new(ino: u64, data: String) -> Data {
        Data{
            ino: ino,
            data: data
        }
    }

    pub fn data(&self) -> &str {
        return &self.data;
    }
}

pub enum Error {
    InternalError
}

impl AllDataStruct {
    pub fn new(all_data: HashMap<u64, Data>) -> AllDataStruct {
        AllDataStruct {
            all_data: all_data
        }
    }

    pub fn all_data(&self, ino: u64) -> Option<&Data> {
        match self.all_data.get(&ino) {
            Some(data) => return Some(data),
            None => return None
        }
    }

    pub fn update_data(&mut self, ino: u64, data: Data) -> Result<(), Error> {
        self.all_data.insert(ino, data);
        return Ok(());
    }
}
impl AllData for AllDataStruct {}