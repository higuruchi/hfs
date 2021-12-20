use crate::{
    externalinterface::{fuse, yaml_image},
    interfaceadapter::{controller, file_repository},
    usecase,
    config
};

pub fn initialize(config: config::Config) -> Result<impl fuse::Fuse , ()> {
    let yaml_image = yaml_image::new();
    let file_repository = file_repository::new(yaml_image);
    let usecase = usecase::new(file_repository);
    let controller = controller::new(usecase);
    let fuse = fuse::new(config, controller);

    return Ok(fuse);
}