#[derive(Debug, Clone)]
pub struct Attr {
    pub ino: u64,
    pub size: u64,
    pub name: String,
    // pub blocks: u32,
    pub atime: SystemTime,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    pub kind: FileType,
    pub perm: u16,
    pub nlink: u32,
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
    atime: SystemTime,
    mtime: SystemTime,
    ctime: SystemTime,
    nlink: u32
) -> Attr {
    Attr {
        ino: ino,
        size: size,
        name: name,
        kind: kind,
        perm: perm,
        uid: uid,
        gid: gid,
        atime: atime,
        mtime: mtime,
        ctime: ctime,
        nlink: nlink
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

    pub fn mtime(&self) -> SystemTime {
        self.mtime
    }

    pub fn ctime(&self) -> SystemTime {
        self.ctime
    }

    pub fn nlink(&self) -> u32 {
        self.nlink
    }

    pub fn perm_mut(&mut self) -> &mut u16 {
        &mut self.perm
    }

    pub fn uid_mut(&mut self) -> &mut u32 {
        &mut self.uid
    }

    pub fn gid_mut(&mut self) -> &mut u32 {
        &mut self.gid
    }

    pub fn size_mut(&mut self) -> &mut u64 {
        &mut self.size
    }

    pub fn atime_mut(&mut self) -> &mut SystemTime {
        &mut self.atime
    }

    pub fn mtime_mut(&mut self) -> &mut SystemTime {
        &mut self.mtime
    }

    pub fn ctime_mut(&mut self) -> &mut SystemTime {
        &mut self.ctime
    }

    pub fn nlink_mut(&mut self) -> &mut u32 {
        &mut self.nlink
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

    pub fn new(sec: u64, nsec: u32) -> SystemTime {
        SystemTime(sec, nsec)
    }

    pub fn as_secs(&self) -> u64 {
        return self.0
    }

    pub fn subsec_nanos(&self) -> u32 {
        return self.1
    }
}