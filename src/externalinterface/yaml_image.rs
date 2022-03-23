extern crate yaml_rust;

use std::path;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Write;
use std::collections::HashMap;
use std::env;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use crate::entity::{
    self,
    attr,
    data,
    entry
};
use crate::interfaceadapter::worker;
use anyhow::Result;

#[derive(Debug)]
pub struct YAMLImageStruct {
    entry: path::PathBuf,
    attr: path::PathBuf,
    data: path::PathBuf
}

const ATTR:         &str = "attr";
const DATA:         &str = "data";
const ENTRY:        &str = "entry";
const INO:          &str = "ino";
const FILE_TYPE:    &str = "file-type";
const PARENT_INO:   &str = "parent-ino";
const NAME:         &str = "name";
const METADATA:     &str = "metadata";
const TIME:         &str = "time";
const FILES:        &str = "files";
const SIZE:         &str = "size";
const UID:          &str = "uid";
const GID:          &str = "gid";
const PERM:         &str = "perm";
const ATIME:        &str = "atime";
const MTIME:        &str = "mtime";
const CTIME:        &str = "ctime";
const NLINK:        &str = "nlink";
const DEL:          &str = "del";

const ATTR_DEFAULT_PATH: &str = "/etc/attr.yaml";
const ENTRY_DEFAULT_PATH: &str = "/etc/entry.yaml";
const DATA_DEFAULT_PATH: &str = "/etc/data.yaml";

const DIRECTORY: u64 = 0;
const TXTFILE: u64 = 1;

impl worker::File for YAMLImageStruct {
    fn init(&mut self, path: &path::Path) -> Result<(u64, attr::AttrsStruct, entry::EntriesStruct, data::AllDataStruct)> {
        self.load_image(path)?;
        let entries = entry::EntriesStruct::new(self.load_entry()?);
        let (attrs_res, next_ino) = self.load_attr();
        let attrs = attr::AttrsStruct::new(attrs_res?);
        let data = data::AllDataStruct::new(self.load_data()?);
        
        Ok((next_ino, attrs, entries, data))
    }

    fn write_data(&self, ino: u64, data: &str) -> Result<()> {
        let data_path = match self.data.to_str() {
            Some(data_path) => data_path,
            None => return Err(entity::Error::InternalError.into())
        };

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(data_path)?;

        file.write_all(format!("- ino: {}\n  data: {:?}\n", ino, data).as_bytes())?;
        return Ok(());
    }

    fn update_attr(&self, attr: &attr::Attr) -> Result<()> {
        let attr_path = match self.attr.to_str() {
            Some(attr_path) => attr_path,
            None => return Err(entity::Error::InternalError.into())
        };
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(attr_path)?;
        let file_type = match attr.file_type() {
            attr::FileType::TextFile => 1,
            attr::FileType::Directory => 0
        };

        file.write_all(
            format!(
                "- ino: {}\n  name: {}\n  file-type: {}\n  size: {}\n  uid: {}\n  gid: {}\n  perm: 0o{:o}\n  atime: \"{}.{}\"\n  mtime: \"{}.{}\"\n  ctime: \"{}.{}\"\n  nlink: {}\n",
                attr.ino(),
                attr.name(),
                file_type,
                attr.size(),
                attr.uid(),
                attr.gid(),
                attr.perm(),
                attr.atime.as_secs(),
                attr.atime.subsec_nanos(),
                attr.mtime.as_secs(),
                attr.mtime.subsec_nanos(),
                attr.ctime.as_secs(),
                attr.ctime.subsec_nanos(),
                attr.nlink()
            ).as_bytes()
        )?;
        return Ok(());
    }

    fn del_attr(&self, ino: u64) -> Result<()> {
        let attr_path = match self.attr.to_str() {
            Some(attr_path) => attr_path,
            None => return Err(entity::Error::InternalError.into())
        };
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(attr_path)?;
        file.write_all(
            format!(
                "- ino: {}\n  del: {}\n",
                ino,
                true
            ).as_bytes()
        )?;

        Ok(())
    }

    fn del_data(&self, ino: u64) -> Result<()> {
        let data_path = match self.data.to_str() {
            Some(data_path) => data_path,
            None => return Err(entity::Error::InternalError.into())
        };

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(data_path)?;

        file.write_all(format!("- ino: {}\n  del: {}\n", ino, true).as_bytes())?;
        return Ok(());    
    }

    fn update_entry(&self, ino: u64, child_inos: &Vec<entry::Entry>) -> Result<()> {
        let entry_path = match self.entry.to_str() {
            Some(entry_path) => entry_path,
            None => return Err(entity::Error::InternalError.into())
        };
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(entry_path)?;

        file.write_all(format!("- ino: {}\n  files:\n", ino).as_bytes())?;
        for entry in child_inos {
            file.write_all(format!("    - {}\n", entry.child_ino()).as_str().as_bytes())?;
        }

        return Ok(());
    }
}

impl YAMLImageStruct {
    pub fn new() -> impl worker::File {
        YAMLImageStruct{
            attr: path::PathBuf::from(ATTR_DEFAULT_PATH),
            entry: path::PathBuf::from(ENTRY_DEFAULT_PATH),
            data: path::PathBuf::from(DATA_DEFAULT_PATH)
        }
    }
    
    fn load_image(&mut self, path: &path::Path) -> Result<()> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(e.into())
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {},
            Err(e) => return Err(e.into())
        };
        let docs = match YamlLoader::load_from_str(&config) {
            Ok(docs) => docs,
            Err(e) => return Err(e.into())
        };

        let mut attr = match &docs[0][ATTR] {
            Yaml::String(s) => s.clone(),
            _ => ATTR_DEFAULT_PATH.to_string()
        };

        let mut entry = match &docs[0][ENTRY] {
            Yaml::String(s) => s.clone(),
            _ => ENTRY_DEFAULT_PATH.to_string()
        };
        
        let mut data = match &docs[0][DATA] {
            Yaml::String(s) => s.clone(),
            _ => DATA_DEFAULT_PATH.to_string()
        };

        let current_dir = match env::current_dir()?.as_os_str().to_str() {
            Some(path) => String::from(path),
            None => String::from("/")
        };

        if *&attr.chars().nth(0).unwrap() == '.' {
            attr.replace_range(..1, &current_dir)
        }

        if *&entry.chars().nth(0).unwrap() == '.' {
            entry.replace_range(..1, &current_dir)
        }

        if *&data.chars().nth(0).unwrap() == '.' {
            data.replace_range(..1, &current_dir)
        }

        self.attr = path::PathBuf::from(attr);
        self.entry = path::PathBuf::from(entry);
        self.data = path::PathBuf::from(data);

        println!("{:?}\n{:?}\n{:?}\n", self.attr, self.entry, self.data);

        return Ok(());
    }

    fn load_entry(&self) -> Result<HashMap<u64, Vec<entry::Entry>>> {
        let mut file = match File::open(&self.entry) {
            Ok(file) => file,
            Err(e) => {
				return Err(e.into());
			}
        };

        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {}
            Err(e) => return Err(e.into())
        }
        let docs = match YamlLoader::load_from_str(&config) {
            Ok(docs) => docs,
            Err(e) => return Err(e.into())
        };
        let mut entrie_hash = HashMap::new();

        for entry_data in docs[0].as_vec().unwrap() {
            let mut entries = Vec::new();
            let ino = match &entry_data[INO] {
                Yaml::Integer(i) => *i as u64,
                _ => return Err(entity::Error::InvalidINO.into())
            };

            match &entry_data[FILES] {
                Yaml::Array(child_inos_data) => {
                    for child_ino_data in child_inos_data {
                        let child_ino = match child_ino_data {
                            Yaml::Integer(i) => *i as u64,
                            _ => return Err(entity::Error::InvalidINO.into())
                        };

                        entries.push(entry::Entry::new(child_ino));
                    }
                },
                _ => {}
            }

            entrie_hash.insert(ino, entries);
        }

        return Ok(entrie_hash);
    }

    fn load_attr(&self) -> (Result<HashMap<u64, attr::Attr>>, u64) {
        let mut file = match File::open(&self.attr) {
            Ok(file) => file,
            Err(e) => return (Err(e.into()), 0)
        };
        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {}
            Err(e) => return (Err(e.into()), 0)
        };
        let docs = match YamlLoader::load_from_str(&config) {
            Ok(docs) => docs,
            Err(e) => return (Err(e.into()), 0)
        };
        let mut attrs_hash = HashMap::new();
        let mut next_ino = 0;
        
        for attr_data in docs[0].as_vec().unwrap() {
            let ino = match &attr_data[INO] {
                Yaml::Integer(i) => *i as u64,
                _ => return (Err(entity::Error::InvalidINO.into()), 0)
            };

            match &attr_data[DEL] {
                Yaml::Boolean(flg) => {
                    if *flg {
                        attrs_hash.remove(&ino);
                        continue;
                    }
                },
                _ => {}
            }

            let name = match &attr_data[NAME] {
                Yaml::String(s) => s.clone(),
                _ => return (Err(entity::Error::InvalidName.into()), 0)
            };
            let file_type = match &attr_data[FILE_TYPE] {
                Yaml::Integer(i) =>{
                    match *i as u64 {
                        DIRECTORY => attr::FileType::Directory,
                        TXTFILE => attr::FileType::TextFile,
                        _ => attr::FileType::TextFile
                    }
                },
                _ => return (Err(entity::Error::InvalidFileType.into()), 0)
            };
            let size = match &attr_data[SIZE] {
                Yaml::Integer(i) => {
                    *i as u64
                },
                _ => return (Err(entity::Error::InvalidSize.into()), 0)
            };
            let uid = match &attr_data[UID] {
                Yaml::Integer(i) => *i as u32,
                _ => return (Err(entity::Error::InvalidUID.into()), 0)
            };
            let gid = match &attr_data[GID] {
                Yaml::Integer(i) => *i as u32,
                _ => return (Err(entity::Error::InvalidGID.into()), 0)
            };
            let perm = match &attr_data[PERM] {
                Yaml::Integer(i) => *i as u16,
                _ => return (Err(entity::Error::InvalidPERM.into()), 0)
            };
            let atime = match &attr_data[ATIME] {
                Yaml::String(s) => {
                    let epoc_string = s.clone();
                    let epoc_vec: Vec<&str> = epoc_string.split('.').collect();
                    let secs: u64 = epoc_vec[0].parse().unwrap();
                    let nanos: u32 = epoc_vec[1].parse().unwrap();

                    attr::SystemTime(secs,nanos)
                },
                _ => return (Err(entity::Error::InvalidAtime.into()), 0)
            };
            let mtime = match &attr_data[MTIME] {
                Yaml::String(s) => {
                    let epoc_string = s.clone();
                    let epoc_vec: Vec<&str> = epoc_string.split('.').collect();
                    let secs: u64 = epoc_vec[0].parse().unwrap();
                    let nanos: u32 = epoc_vec[1].parse().unwrap();

                    attr::SystemTime(secs,nanos)
                },
                _ => return (Err(entity::Error::InvalidAtime.into()), 0)
            };
            let ctime = match &attr_data[CTIME] {
                Yaml::String(s) => {
                    let epoc_string = s.clone();
                    let epoc_vec: Vec<&str> = epoc_string.split('.').collect();
                    let secs: u64 = epoc_vec[0].parse().unwrap();
                    let nanos: u32 = epoc_vec[1].parse().unwrap();

                    attr::SystemTime(secs,nanos)
                },
                _ => return (Err(entity::Error::InvalidAtime.into()), 0)
            };

            let nlink = match &attr_data[NLINK] {
                Yaml::Integer(i) => *i as u32,
                _ => return (Err(entity::Error::InvalidNlink.into()), 0)
            };

            if ino >= next_ino {
                next_ino = ino + 1;
            }
            
            attrs_hash.insert(ino, attr::Attr::new(ino, size, name, file_type, perm, uid, gid, atime, mtime, ctime, nlink));
        }

        return (Ok(attrs_hash), next_ino);
    }
    
    fn load_data(&self) -> Result<HashMap<u64, data::Data>> {
        let mut file = match File::open(&self.data) {
            Ok(file) => file,
            Err(e) => return Err(e.into())
        };

        let mut config = String::new();
        match file.read_to_string(&mut config) {
            Ok(_) => {}
            Err(e) => return Err(e.into())
        };
        
        let docs = match YamlLoader::load_from_str(&config) {
            Ok(docs) => docs,
            Err(e) => return Err(e.into())
        };

        let mut data_hash = HashMap::new();

        for data in docs[0].as_vec().unwrap() {
            let ino = match &data[INO] {
                Yaml::Integer(i) => *i as u64,
                _ => return Err(entity::Error::InvalidINO.into())
            };

            match &data[DEL] {
                Yaml::Boolean(flg) => {
                    if *flg {
                        data_hash.remove(&ino);
                        continue;
                    }
                },
                _ => {}
            }

            let text_data = match &data[DATA] {
                Yaml::String(s) => s.clone(),
                _ => return Err(entity::Error::InvalidData.into())
            };
            
            data_hash.insert(ino, data::Data::new(ino, text_data));
        }

        return Ok(data_hash);
    }
}
