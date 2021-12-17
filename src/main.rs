use clap::Parser;
use hfs::config;
use hfs::{di, externalinterface::fuse::Fuse};

fn main() {
    let config = config::Config::parse();
    let fs = match di::initialize(config) {
        Ok(fs) => fs,
        Err(()) => panic!("Initialize error")
    };
    
    fs.init();
}