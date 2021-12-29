#[derive(Debug)]
pub struct Data {
    pub ino: u64,
    data: String
}

pub fn new(ino: u64, data: String) -> Data {
    Data{
        ino: ino,
        data: data
    }
}

impl Data {
    pub fn data(&self) -> &str {
        return &self.data;
    }
}
