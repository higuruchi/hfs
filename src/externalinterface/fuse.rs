use std::fs::File;
use crate::{interfaceadapter::controller, config};
use fuse::{
    Filesystem,
    ReplyEntry,
    ReplyAttr,
    ReplyData,
    ReplyDirectory,
    Request,
    ReplyWrite,
    ReplyCreate,
    ReplyEmpty
};
use std::ffi::OsStr;
use time;
use libc;

struct FuseStruct <C: controller::Controller>{
    config: String,
    mountpoint: String,
    controller: C
}

pub fn new<C>(config: config::Config, controller: C) -> impl Filesystem
    where C: controller::Controller,
{
    FuseStruct{
        config: config.config_path,
        mountpoint: config.mountpoint,
        controller: controller
    }
}

impl<C: controller::Controller> Filesystem for FuseStruct<C> {
    fn init(&mut self, _req: &Request<'_>) -> Result<(), libc::c_int> {

        match self.controller.init(&self.config) {
            Ok(_) => {
				println!("Initialized!");
				return Ok(());
			},
            Err(e) => {
				println!("Failed Initialized!: {}", e);
				return Err(libc::ENOENT)
			}
        }
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry){

        match self.controller.lookup(parent, name) {
            Some(attr) => reply.entry(&time::Timespec{sec: 1, nsec: 0}, &attr , 0),
            None => reply.error(libc::ENOENT)
        };
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
		
        match self.controller.getattr(ino) {
            Some(attr) => reply.attr(&time::Timespec{sec: 1, nsec: 0}, &attr),
            None => reply.error(libc::ENOENT)
        }
		return;
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
		if offset == 0 {

			let files_data = match self.controller.readdir(ino) {
				Some(files_data) => files_data,
				None => return reply.error(libc::ENOENT)
			};
			
			for (i, file_data) in files_data.iter().enumerate() {
				let offset: i64 = (i + 1).try_into().unwrap();
				let ino = file_data.0;
				let kind = file_data.2;
				let name = file_data.1;

				let full = reply.add(ino, offset, kind, name);
				if full {
					break;
				}
			}
		}
		reply.ok();
	}

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, size: u32, reply: ReplyData){
        let data = match self.controller.read(ino, offset, size as u64) {
            Some(data) => data,
            None => return  reply.error(libc::ENOENT)
        };

        reply.data(data);
    } 

    fn write(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _fh: u64,
        offset: i64,
        data: &[u8],
        flags: u32,
        reply: ReplyWrite
    ) {
        match self.controller.write(ino, offset as u64, data) {
            Ok(size) => reply.written(size),
            Err(_) => reply.error(libc::ENOENT)
        }
    }

    fn setattr(
        &mut self,
        _req: &Request<'_>,
        ino: u64, // 更新対象のinode番号
        mode: Option<u32>, // アクセス権
        uid: Option<u32>, // ファイル所有者のUID
        gid: Option<u32>, // ファイル所有グループのUID
        size: Option<u64>, // ファイルサイズ
        atime: Option<time::Timespec>, // 最終アクセス時刻
        mtime: Option<time::Timespec>, // 最終更新時刻
        _fh: Option<u64>,
        crtime: Option<time::Timespec>, // mac用
        chgtime: Option<time::Timespec>, // mac用
        bkuptime: Option<time::Timespec>, // mac用
        flags: Option<u32>, // mac用
        reply: ReplyAttr
    ) {
        match self.controller.setattr(ino, mode, uid, gid, size, atime, mtime) {
            Ok(attr) => reply.attr(&time::Timespec{sec: 1, nsec: 0}, &attr),
            Err(_) => reply.error(libc::ENOENT)
        }
    }

    fn create(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32,
        reply: ReplyCreate
    ) {
        // すでにあるかチェック
        // なかった場合modeを指定して作成
        match self.controller.lookup(parent, name) {
            Some(attr) => reply.created(&time::Timespec{sec: 1, nsec: 0}, &attr , 0, 0, 0),
            None => {
                match self.controller.create(parent, name, mode, flags) {
                    Ok(attr) => reply.created(&time::Timespec{sec: 1, nsec: 0}, &attr , 0, 0, 0),
                    Err(_) => reply.error(libc::ENOENT)
                }
            }
        }
    }

    fn unlink(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        reply: ReplyEmpty
    ) {
        match self.controller.unlink(parent, name) {
            Ok(_) => reply.ok(),
            Err(_) => reply.error(libc::ENOENT)
        }
    }

    fn forget(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _nlookup: u64
    ) {
        self.controller.forget(_ino, _nlookup);
    }

    fn mkdir(
        &mut self,
        req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        reply: ReplyEntry
    ) {
        // 親ディレクトリのSGIDがONの場合、子にSGIDを追加
        // 親のスティッキービットがONの場合、子にスティッキービットを追加

        match self.controller.mkdir(parent, name, mode) {
            Ok(attr) => reply.entry(&time::Timespec{sec: 1, nsec: 0}, &attr, 0),
            Err(_) => reply.error(libc::ENOENT)
        };
    }

    fn rmdir(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        reply: ReplyEmpty
    ) {
        match self.controller.rmdir(parent, name) {
            Ok(_) => reply.ok(),
            Err(_) => reply.error(libc::ENOENT)
        }
    }

    fn rename(
        &mut self,
        _req: &Request,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
        reply: ReplyEmpty
    ) {
        match self.controller.rename(parent, name, newparent, newname) {
            Ok(_) => reply.ok(),
            Err(_) => reply.error(libc::ENOENT)
        }
    }


}

