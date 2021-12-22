use clap::Parser;
use hfs::config;
use hfs::{di, externalinterface::fuse::Fuse};

fn main() {
    let config = config::Config::parse();
    let mut fs = match di::initialize(config) {
        Ok(fs) => fs,
        Err(()) => panic!("Initialize error")
    };

    fs.init();
    fs.lookup();
    fs.getattr();
}