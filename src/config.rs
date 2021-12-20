use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "hfs",
    version = "0.0.1",
    author = "higuruchi",
)]
pub struct Config {
    #[clap(short, long)]
    pub config_path: String
}