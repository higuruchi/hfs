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

// use fuse::{
//     Filesystem,
//     ReplyEntry,
//     ReplyAttr,
//     ReplyData,
//     ReplyDirectory,
//     Request,
//     FileAttr
// };

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
    hfs_entry: HashMap<i64, Vec<Entry>>,
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

            let ino = match &node["node"]["ino"] {
                Yaml::Integer(i) => *i,
                _ => -1
            };

            let parent_ino = match &node["node"]["parent-ino"] {
                Yaml::Integer(i) => *i,
                _ => -1
            };

            let data = match &node["node"]["data"] {
                Yaml::String(s) => s.clone(),
                _ => String::new()
            };

            let name = match &node["node"]["name"] {
                Yaml::String(s) => s.clone(),
                _ => String::new()
            };

            self.append_attr(ino, data.len() as i64, name);

            match file_type {
                FileType::Directory => {
                    self.append_entry(ino, parent_ino, files);
                }
                FileType::Text => {
                    self.append_data(ino, data);
                }
                FileType::None => {
                    panic!("initialize error");
                }
            }            
        }
        println!("{:?}", self);
    }

    fn append_entry(&mut self, ino: i64, parent_ino: i64, files: Vec<i64>) {
        for child_ino in files {
            let entry = entry::new(
                ino,
                parent_ino,
                child_ino
            );

            match self.hfs_entry.get(&ino) {
                Some(vec) => {
                    let mut new_vec = vec.clone();
                    new_vec.push(entry);
                    self.hfs_entry.insert(ino, new_vec);
                },
                None => {
                    let mut vec = Vec::new();
                    vec.push(entry);
                    self.hfs_entry.insert(ino, vec);
                }
            };
        }
    }

    fn append_data(&mut self, ino: i64, data: String) {
        let data = data::new(
            ino,
            data.to_string()
        );

        self.hfs_data.insert(ino, data);
    }

    fn append_attr(&mut self, ino: i64, size: i64, name: String) {
        let attr = attr::new(
            ino,
            size,
            name
        );

        self.hfs_attr.insert(ino, attr);
    }
}


// impl Filesystem for HFS {
//     fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {

//         let hfs_attr = self.hfs_attr.get(&)
//         file_attr = FileAttr {
//             ino:
//         }
//     }

//     fn getattr(&mut self, _req, &request, ino: u64, reply: ReplyAttr) {

//     }

//     fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, reply: ReplyData) {

//     }

//     fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {

//     }
// }