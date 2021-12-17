use crate::{
    externalinterface::fuse,
    interfaceadapter::{controller, file_repository},
    usecase,
    config
};

pub fn initialize(config: config::Config) -> Result<impl fuse::Fuse , ()> {
    let file_repository = file_repository::new();
    let usecase = usecase::new(file_repository);
    let controller = controller::new(usecase);
    let fuse = fuse::new(config, controller);

    return Ok(fuse);
}