use clap::Parser;
use hfs::hfs;

fn main() {
    let args = hfs::Args::parse();

    println!("{}", args.config_path)
}