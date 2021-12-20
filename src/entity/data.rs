#[derive(Debug)]
pub struct Data {
    pub ino: i64,
    data: String
}

pub fn new(ino: i64, data: String) -> Data {
    Data{
        ino: ino,
        data: data
    }
}