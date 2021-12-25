use std::fs::File;
use crate::{interfaceadapter::controller, config};
use fuse::{
    Filesystem,
    ReplyEntry,
    ReplyAttr,
    ReplyData,
    ReplyDirectory,
    Request
};
use std::ffi::OsStr;
use time;
use libc;

struct FuseStruct <C: controller::Controller>{
    config: String,
    mountpoint: String,
    controller: C
}

// pub trait Fuse {
//     fn init(&mut self);
//     fn lookup(&self);
//     fn getattr(&self);
//     fn readdir(&self);
// }

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
            Err(_) => {
				println!("Failed Initialized!");
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
		println!("offset:{:?}", offset);
		if offset == 0 {

			println!("called readdir: {:?}", self.controller.readdir(ino));
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
}

// impl<C: controller::Controller> Fuse for FuseStruct<C> {
//     fn init(&mut self) {
//         self.controller.init(&self.config);
//     }

//     fn lookup(&self) {
//         let attr = self.controller.lookup(0, OsStr::new("file1"));
//         println!("attr : {:?}", attr);
//     }

//     fn getattr(&self) {
//         let attr = self.controller.getattr(1);
//         println!("attr : {:?}", attr);
//     }

//     fn readdir(&self) {
//         let entry = self.controller.readdir(0);
//         println!("entry : {:?}", entry);
//     }
// }
