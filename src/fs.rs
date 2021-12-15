extern crate yaml_rust;

mod data;
mod entry;
mod attr;

use clap::Parser;
use std::fs::File;
use std::collections::HashMap;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use std::io::prelude::*;
use crate::fs::{attr::Attr, data::Data, entry::Entry};

#[derive(Parser, Debug)]
#[clap(
    name = "hfs",
    version = "0.0.1",
    author = "higuruchi",
)]
pub struct Args {
    #[clap(short, long)]
    pub config_path: String,
}

#[derive(Debug)]
pub struct HFS {
    args: Args,
    hfs_attr: HashMap<i64, Attr>,
    hfs_entry: HashMap<i64, Entry>,
    hfs_data: HashMap<i64, Data>
}

#[derive(Debug, Copy, Clone)]
pub enum FileType {
    Directory,
    Text,
    None
}


pub fn new(args: Args) -> HFS {
    HFS {
        args: args,
        hfs_attr: HashMap::new(),
        hfs_entry: HashMap::new(),
        hfs_data: HashMap::new()
    }
}

impl HFS {
    pub fn initialize(&mut self) {
        let mut file = File::open(&self.args.config_path).expect("file not found");
        let mut config = String::new();

        file.read_to_string(&mut config).expect("something went wrong reading the file");

        let docs = YamlLoader::load_from_str(&config).unwrap();

        for node in docs[0].as_vec().unwrap() {
            let file_type = match node["node"]["file-type"].as_str().unwrap() {
                "directory" => FileType::Directory,
                "text" => FileType::Text,
                _ => FileType::None
            };
            let ino = match &node["node"]["ino"] {
                Yaml::Integer(i) => *i,
                _ => -1
            };
            let data = match &node["node"]["data"] {
                Yaml::String(s) => s.clone(),
                _ => String::new()
            };
            let parent_ino = match &node["node"]["parent-ino"] {
                Yaml::Integer(i) => *i,
                _ => -1
            };
            let files = match &node["node"]["files"] {
                Yaml::Array(array) => {
                    let mut inos = Vec::new();

                    for ino in array {
                        inos.push(match ino {
                            Yaml::Integer(i) => *i,
                            _ => -1
                        });
                    }

                    inos
                },
                _ => vec!()
            };
            let file_name = match &node["node"]["data"] {
                Yaml::String(s) => s.clone(),
                _ => String::new()
            };
            let attr = attr::new(
                ino,
                data.len() as i64
            );

            self.hfs_attr.insert(ino, attr);

            match file_type {
                FileType::Directory => {
                    for child_ino in files {
                        let entry = entry::new(
                            ino,
                            parent_ino,
                            child_ino,
                            file_name.clone(),
                            file_type,
                        );

                        self.hfs_entry.insert(ino, entry);
                    }
                }
                FileType::Text => {
                    let data = data::new(
                        ino,
                        data.to_string()
                    );
                    
                    self.hfs_data.insert(ino, data);
                }
                FileType::None => {
                    panic!("initialize error");
                }
            }
        }
        println!("{:?}", self);
    }
}
