#[derive(Debug)]
pub struct Attr {
    pub ino: u64,
    pub size: u64,
    pub name: String,
    // pub blocks: u32,
    pub atime: SystemTime,
    // pub mtime: SystemTime,
    // pub ctime: SystemTime,
    // pub crtime: SystemTime,
    pub kind: FileType,
    pub perm: u16,
    // pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    // pub rdev: u32,
    // pub flags: u32,
}

#[derive (Clone, Copy, Debug)]
pub enum FileType {
    Directory,
    TextFile
}

#[derive(Clone, Copy, Debug)]
pub struct SystemTime(pub u64, pub u32);

pub fn new(
    ino: u64,
    size: u64,
    name: String,
    kind: FileType,
    perm: u16,
    uid: u32,
    gid: u32,
    atime: SystemTime
) -> Attr {
    Attr {
        ino: ino,
        size: size,
        name: name,
        kind: kind,
        perm: perm,
        uid: uid,
        gid: gid,
        atime: atime
    }
}

impl Attr {
    pub fn ino(&self) -> u64 {
        self.ino
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn file_type(&self) -> FileType {
        self.kind
    }

    pub fn perm(&self) -> u16 {
        self.perm
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }

    pub fn kind(&self) -> FileType {
        self.kind
    }

    pub fn atime(&self) -> SystemTime {
        self.atime
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        let now = std::time::SystemTime::now();
        if let Ok(epoch) = now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            SystemTime(epoch.as_secs(), epoch.subsec_nanos())
        } else {
            SystemTime(0, 0)
        }
    }
    pub fn as_secs(&self) -> u64 {
        return self.0
    }

    pub fn subsec_nanos(&self) -> u32 {
        return self.1
    }
}