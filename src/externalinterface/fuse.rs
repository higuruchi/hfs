use std::fs::File;
use crate::{interfaceadapter::controller, config};
use fuse::{
    Filesystem,
    ReplyEntry,
    ReplyAttr,
    ReplyData,
    ReplyDirectory,
    Request,
    ReplyWrite
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

    fn write(&mut self, _req: &Request<'_>, ino: u64, _fh: u64, offset: i64, data: &[u8], flags: u32, reply: ReplyWrite) {
        let size = match self.controller.write(ino, offset as u64, data) {
            Ok(data) => data,
            Err(_) => return reply.error(libc::ENOENT)
        };
        reply.written(size)
    }
}

