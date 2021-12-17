use crate::usecase;
use std::path::Path;

struct ControllerStruct<U: usecase::Usecase> {
    usecase: U
}

pub trait Controller {
    fn init(&self, config: String) -> Result<(), ()>;
}

pub fn new<U>(usecase: U) -> impl Controller
    where U: usecase::Usecase
{
    ControllerStruct{
        usecase: usecase
    }
}

impl<U: usecase::Usecase> Controller for ControllerStruct<U> {
    fn init(&self, config: String) -> Result<(), ()> {
        self.usecase.init(&Path::new(&config));
        return Ok(());
    }
}