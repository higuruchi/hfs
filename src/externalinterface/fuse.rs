use std::fs::File;
use crate::{interfaceadapter::controller, config};

struct FuseStruct <C: controller::Controller>{
    config: String,
    controller: C
}

pub trait Fuse {
    // fn init(&mut self, _req: &Request<'_>) -> Result<(), c_int>;
    fn init(&self);
}

pub fn new<C>(config: config::Config, controller: C) -> impl Fuse
    where C: controller::Controller,
{
    FuseStruct{
        config: config.config_path,
        controller: controller
    }
}

impl<C: controller::Controller> Fuse for FuseStruct<C> {
    // fn init(&mut self, _req: &Request<'_>) -> Result<(), c_int> {
    //     self.controller.init()
    // }
    fn init(&self) {
        self.controller.init(String::from("hello world!"));
    }
}