use crate::{
    // externalinterface::{fuse, yaml_image},
    externalinterface,
    interfaceadapter::{controller, file_repository},
    usecase,
    config
};
use fuse;

pub fn initialize(config: config::Config) -> Result<impl fuse::Filesystem , ()> {
    let yaml_image = externalinterface::yaml_image::new();
    let file_repository = file_repository::new(yaml_image);
    let usecase = usecase::new(file_repository);
    let controller = controller::new(usecase);
    let fuse = externalinterface::fuse::new(config, controller);

    return Ok(fuse);
}