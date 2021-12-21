use std::fs::File;
use crate::{interfaceadapter::controller, config};
// use fuse::{
//     Filesystem,
//     ReplyEntry,
//     ReplyAttr,
//     ReplyData,
//     ReplyDirectory,
//     Request
// };
use std::ffi::OsStr;

struct FuseStruct <C: controller::Controller>{
    config: String,
    controller: C
}

pub trait Fuse {
    fn init(&mut self);
    fn lookup(&self);
}

pub fn new<C>(config: config::Config, controller: C) -> impl Fuse
    where C: controller::Controller,
{
    FuseStruct{
        config: config.config_path,
        controller: controller
    }
}

// impl Filesystem for FuseStruct {
//     fn init(&mut self, _req: &Request<'_>) -> Result<(), c_int> {
//         match self.controller.init(&self.config) {
//             Ok() => return Ok(()),
//             Err() => {}
//         };
//         return Ok(());
//     }

//     fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry){

//     }
// }

impl<C: controller::Controller> Fuse for FuseStruct<C> {
    fn init(&mut self) {
        self.controller.init(&self.config);
    }

    fn lookup(&self) {
        let attr = self.controller.lookup(0, OsStr::new("file1"));
        println!("{:?}", attr);
    }
}