#[derive(Debug)]
pub struct Attr {
    pub ino: u64,
    pub size: u64,
    pub name: String,
    // pub blocks: u32,
    // pub atime: SystemTime,
    // pub mtime: SystemTime,
    // pub ctime: SystemTime,
    // pub crtime: SystemTime,
    pub kind: FileType,
    // pub perm: u16,
    // pub nlink: u32,
    // pub uid: u32,
    // pub gid: u32,
    // pub rdev: u32,
    // pub flags: u32,
}

#[derive (Clone, Copy, Debug)]
pub enum FileType {
    Directory,
    TextFile
}

pub fn new(
    ino: u64,
    size: u64,
    name: String,
    kind: FileType
) -> Attr {
    Attr {
        ino: ino,
        size: size,
        name: name,
        kind: kind
    }
}

impl Attr {
    pub fn ino(&self) -> u64 {
        return self.ino;
    }

    pub fn size(&self) -> u64 {
        return self.size;
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }

    pub fn file_type(&self) -> FileType {
        return self.kind;
    }
}