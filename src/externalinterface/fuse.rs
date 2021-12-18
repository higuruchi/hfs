use std::fs::File;
use crate::{interfaceadapter::controller, config};

struct FuseStruct <C: controller::Controller>{
    config: String,
    controller: C
}

pub trait Fuse {
    // fn init(&mut self, _req: &Request<'_>) -> Result<(), c_int>;
    fn init(&mut self);
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
    fn init(&mut self) {
        self.controller.init(String::from("/etc/image.yaml"));
    }
}