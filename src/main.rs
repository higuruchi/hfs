use clap::Parser;
use hfs::fs;

fn main() {
    let args = fs::Args::parse();
    let mut fs = fs::new(args);
    
    fs.initialize();
}