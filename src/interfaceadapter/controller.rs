use crate::usecase;
use std::path::Path;
use std::ffi::OsStr;
use crate::entity::attr;

struct ControllerStruct<U: usecase::Usecase> {
    usecase: U
}

pub trait Controller {
    fn init(&mut self, config: &String) -> Result<(), ()>;
    fn lookup(&self, parent: u64, name: &OsStr) -> Option<&attr::Attr>;
}

pub fn new<U>(usecase: U) -> impl Controller
    where U: usecase::Usecase
{
    ControllerStruct{
        usecase: usecase
    }
}

impl<U: usecase::Usecase> Controller for ControllerStruct<U> {
    fn init(&mut self, config: &String) -> Result<(), ()> {
        self.usecase.init(&Path::new(config));
        return Ok(());
    }

    fn lookup(&self, parent: u64, name: &OsStr) -> Option<&attr::Attr> {
        match self.usecase.lookup(parent, name) {
            Some(attr) => Some(attr),
            None => None
        }
    }
}