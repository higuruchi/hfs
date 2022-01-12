use clap::Parser;
use hfs::config;
use hfs::di;
use fuse;
use std::ffi::OsStr;

fn main() {
    env_logger::init();
    let config = config::Config::parse();

    // 後ほど修正
    let mountpoint = config.mountpoint.clone();

    let mut fs = match di::initialize(config) {
        Ok(fs) => fs,
        Err(()) => panic!("Initialize error")
    };

	println!("mounted hfs");
    fuse::mount(fs, &mountpoint, &[]).expect("failed mount");
}
