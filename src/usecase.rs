pub mod repository;

use std::collections::HashMap;
use std::path;
use std::ffi::OsStr;
use anyhow::Result;
use crate::entity::{self, attr, data, entry, lookup_count};

#[derive(Debug)]
struct UsecaseStruct<F: repository::File> {
    next_ino: Option<u64>,
    attr: Option<attr::AttrsStruct>,
    entry: Option<entry::EntriesStruct>,
    data: Option<data::AllDataStruct>,
    lookup_count: Option<lookup_count::LookupCount>,
    file_repository: F
}

pub trait Usecase {
    fn init(&mut self, path: &path::Path) -> Result<()>;
    fn lookup(&mut self, parent: u64, name: &OsStr) -> Option<attr::Attr>;
    fn attr_from_ino(&self, ino: u64) -> Option<&attr::Attr>;
    fn readdir(&mut self, ino: u64) -> Option<Vec<(u64, &str, attr::FileType)>>;
    fn read(&mut self, ino: u64, offset: i64, size: u64) -> Option<&str>;
    fn write(&mut self, ino: u64, offset: u64, data: &str) -> Result<u64>;
    fn setattr(
        &mut self,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<attr::SystemTime>,
        mtime: Option<attr::SystemTime>
    ) -> Result<attr::Attr>;
    fn create(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32
    ) -> Result<attr::Attr>;
    fn unlink(
        &mut self,
        parent: u64,
        name: &OsStr
    ) -> Result<()>;
    fn forget(
        &mut self,
        ino: u64,
        nlookup: u64,
    ) -> Result<()>;
    fn mkdir(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
    ) -> Result<attr::Attr>;
    fn rmdir(
        &mut self,
        parent: u64,
        name: &OsStr,
    ) -> Result<()>;
    fn rename (
        &mut self,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
    ) -> Result<()>;
    fn new_ino(&mut self) -> u64;
}

pub fn new<F>(file_repository: F) -> impl Usecase 
    where F: repository::File
{
    UsecaseStruct{
        next_ino: None,
        attr: None,
        entry: None,
        data: None,
        lookup_count: None,
        file_repository: file_repository
    }
}

impl<F: repository::File> Usecase for UsecaseStruct<F> {
    fn init(&mut self, path: &path::Path) -> Result<()> {
       match self.file_repository.init(path) {
            Ok(files_data) => {
                self.next_ino = Some(files_data.0);
                self.attr = Some(files_data.1);
                self.entry = Some(files_data.2);
                self.data = Some(files_data.3);
                self.lookup_count = Some(lookup_count::LookupCount::new());
                return Ok(());
            },
            Err(e) => return Err(e)
        };
    }

    fn lookup(&mut self, parent: u64, name: &OsStr) -> Option<attr::Attr> {
        // 子供のエントリ
        let entries = match self.entry() {
            Some(entries) => match entries.entry(parent) {
                Some(entry) => entry,
                None => return None
            },
            None => return None
        };

        // 親ディレクトリのエントリからnameの名前を持つ子どをも探索する
        for entry in entries.iter() {
            let child_ino = entry.child_ino();
            let child_attr = match self.attr() {
                Some(attr) => match attr.attr(child_ino) {
                    Some(child_attr) => child_attr,
                    None => return None
                },
                None => return None
            };
            let file_name = match name.to_str() {
                Some(file_name) => file_name,
                None => return None
            };

            if child_attr.name == file_name {
                let lookup_attr_data = child_attr.clone();

                // mutable-----------------------------------
                 match self.lookup_count_mut() {
                    Some(lookup_count) => {
                        lookup_count.update_lookupcount(child_ino);
                        return Some(lookup_attr_data);
                    },
                    None => return None
                };
                // -------------------------------------------
            }
        };
        None
    }

    fn attr_from_ino(&self, ino: u64) -> Option<&attr::Attr> {
        let attr = match self.attr() {
            Some(attr) => attr,
            None => return None
        };

        attr.attr(ino)
    }

    fn readdir(&mut self, ino: u64) -> Option<Vec<(u64, &str, attr::FileType)>> {
        let mut ret_vec = Vec::new();
        let st = attr::SystemTime::now();

        // mutable-----------------------------------
        // atime属性のタイムスタンプを更新
        match self.attr_mut() {
            Some(attr) => attr.update_atime(ino, st),
            None => return None
        };
        // -------------------------------------------

        let attr = match self.attr() {
            Some(attr) => attr,
            None => return None
        };
        // attr.yamlを更新
        match attr.attr(ino) {
            Some(attr_data) => self.file_repository.update_attr(&attr_data),
            None => return None
        };
        let entries = match self.entry() {
            Some(entries) => match entries.entry(ino) {
                Some(entry) => entry,
                None => return None
            },
            None => return None
        };

        for entry in entries.iter() {
            let child_ino = entry.child_ino();
            let child_attr = match attr.attr(child_ino) {
                Some(child_attr) => child_attr,
                None => return None
            };
            let file_name = child_attr.name();
            let file_type = child_attr.file_type();

            ret_vec.push((child_ino, file_name, file_type));
        }

        return Some(ret_vec);
    }
    
    fn read(&mut self, ino: u64, offset: i64, size: u64) -> Option<&str> {
        // mutable-----------------------------------
        // atime属性を更新
        match self.attr_mut() {
            Some(attr) => attr.update_atime(ino, attr::SystemTime::now()),
            None => return None
        };
        // -------------------------------------------
        // attr.yamを更新
        match self.attr() {
            Some(attr) => match attr.attr(ino) {
                Some(attr_data) => {self.file_repository.update_attr(attr_data);},
                None => return None
            },
            None => return None
        }

        match self.data() {
            Some(data) => match data.all_data(ino) {
                Some(data) => {
                    // TODO: offsetを考慮して文字列を抜き出す
                    let end = offset as u64 + size;
                    return Some(data.data());
                },
                None => return None
            },
            None => return None
        };

        // return Some(&text_data[(offset as usize)..(end as usize)]);
    }

    fn write(&mut self, ino: u64, offset: u64, data: &str) -> Result<u64> {
        let mut new_text_data_len: u64 = 0;
        // dataを更新
        // mutable: self.data-----------------------------------
        match self.data_mut() {
            Some(all_data) => match all_data.all_data(ino) {
                Some(old_data) => {
                    let new_text_data = match merge_str(offset, data, old_data.data()) {
                        Ok(new_text_data) => new_text_data,
                        Err(e) => return Err(e.into())
                    };
                    new_text_data_len = new_text_data.len() as u64;
                    all_data.update_data(ino, data::Data::new(ino, new_text_data));
                },
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        }
        // -------------------------------------------
        // atimeを更新
        match self.attr_mut() {
            Some(attr) => {
                let st = attr::SystemTime::now();
                attr.update_size(ino, new_text_data_len);
                attr.update_mtime(ino, st);
                attr.update_ctime(ino, st);
            },
            None => return Err(entity::Error::InternalError.into())
        }
        // attr.yamlファイルを更新
        match self.attr() {
            Some(attr) => match attr.attr(ino) {
                Some(attr_data) => {self.file_repository.update_attr(&attr_data);},
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        }
        // data.yamlファイルを更新
        match self.data() {
            Some(all_data) => match all_data.all_data(ino) {
                Some(data) => {self.file_repository.write_data(ino, data.data());},
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        }


        Ok(new_text_data_len)
    }

    fn setattr(
        &mut self,
        ino: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<attr::SystemTime>,
        mtime: Option<attr::SystemTime>
    ) -> Result<attr::Attr> {
         // TODO:
        // sizeが元のファイルサイズより小さい0以外の値が指定された場合、
        // 残すべきデータは残しつつ、いらないデータがきちんと破壊されるようにする

        // TODO:
        // 元のファイルサイズより大きい値が指定された場合、
        // 間のデータが0(\0)で埋められるようにする
        // let imu_entity = match &self.entity {
        //     Some(entity) => entity,
        //     None => return Err(entity::Error::InternalError.into()) 
        // };

        let mut com_data_size: Result<attr::Compare, attr::Error> = Err(attr::Error::InternalError);

        // atimeを更新
        match self.attr_mut() {
            Some(attr)=> {
                if let Some(n) = mode { attr.update_perm(ino, n as u16); };
                if let Some(n) = uid { attr.update_uid(ino, n); };
                if let Some(n) = gid { attr.update_gid(ino, n); };
                if let Some(n) = size { 
                    com_data_size = attr.cmp_data_size(ino, n as u64);
                    attr.update_size(ino, n); 
                };
                if let Some(n) = atime { attr.update_atime(ino, n); };
                if let Some(n) = mtime { attr.update_mtime(ino, n); };

            },
            None => return Err(entity::Error::InternalError.into())
        }
        // dataを更新
        let mut new_data = String::new();

        // 更新後のdata内のテキストを取得
        match self.data() {
            Some(all_data) => {
                if let Some(n) = size {
                    match com_data_size {
                        Ok(c) =>  {
                            match c {
                                Smaller => {
                                    // ずるしてます
                                    new_data = smaller_data(all_data.all_data(ino).unwrap().data(), n);
                                },
                                Begger => {
                                    // ずるしてます
                                    new_data = begger_data(all_data.all_data(ino).unwrap().data(), n);
                                },
                                Equal => {
                                    // 無駄な処理
                                    // ずるしてます
                                    new_data = all_data.all_data(ino).unwrap().data().to_string();
                                }
                            }
                        },
                        Err(_) => return Err(entity::Error::InternalError.into())

                    }
                }
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // dataの更新
        match self.data_mut() {
            Some(all_data) => {all_data.update_data(ino, data::Data::new(ino, new_data));},
            None => return Err(entity::Error::InternalError.into())
        }

        // atime.yamlを更新
        match self.attr() {
            Some(attr) => match attr.attr(ino) {
                Some(attr_data) => {self.file_repository.update_attr(&attr_data);},
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        }
        // data.yamlを更新
        match self.data() {
            Some(all_data) => match all_data.all_data(ino) {
                Some(data) => {self.file_repository.write_data(ino, data.data());},
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        }


        // 返却
        let attr_data = match self.attr() {
            Some(attr) => match attr.attr(ino) {
                Some(attr_data) => attr_data.clone(),
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        };
        Ok(attr_data)

    }

    fn create(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
        flags: u32
    ) -> Result<attr::Attr> {
        let new_ino = self.new_ino();
        // attrの更新
        match self.attr_mut() {
            Some(attr) => {
                let name_string = match name.to_str() {
                    Some(name) => name.to_string(),
                    None => return Err(entity::Error::InternalError.into())
                };
                let new_attr = attr::Attr::new(
                    new_ino,
                    0,
                    name_string,
                    attr::FileType::TextFile,
                    mode as u16,
                    // TODO::ユーザID グループID固定値
                    1000,
                    1000,
                    attr::SystemTime::now(),
                    attr::SystemTime::now(),
                    attr::SystemTime::now(),
                    1
                );
                attr.inc_size(parent);
                attr.update_attr(new_attr);
            },
            None => return Err(entity::Error::InternalError.into()) 
        }
        // dataの更新
        match self.data_mut() {
            Some(data) => { data.update_data(new_ino, data::Data::new(new_ino, "".to_string())); },
            None => return Err(entity::Error::InternalError.into())
        }
        // entryの更新
        match self.entry_mut() {
            Some(entry) => { entry.insert_child_ino(parent, new_ino); },
            None => return Err(entity::Error::InternalError.into())
        }
        // attr.yamlの更新
        match self.attr() {
            Some(attr) => {
                self.file_repository.update_attr(attr.attr(parent).unwrap())?;
                self.file_repository.update_attr(attr.attr(new_ino).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }
        // data.yamlの更新
        match self.data() {
            Some(all_data) => match all_data.all_data(new_ino) {
                Some(data) => {self.file_repository.write_data(new_ino, data.data());},
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        }
        // entry.yamlの更新
        match self.entry() {
            Some(entry) => {
                self.file_repository.update_entry(new_ino, entry.entry(parent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }

        let attr_data = match self.attr() {
            Some(attr) => match attr.attr(parent) {
                Some(attr_data) => attr_data.clone(),
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        };

        Ok(attr_data)
    }

    fn unlink(
        &mut self,
        parent: u64,
        name: &OsStr
    ) -> Result<()> {
        // unlinkするファイルのino
        let unlink_child_ino = match self.child_ino_from_parent(parent, name) {
            Some(child_ino) => child_ino,
            None => return Err(entity::Error::InternalError.into())
        };
        
        // entryから当該のエントリを削除する
        match self.entry_mut() {
            Some(entry) => entry.remove_child_ino(parent, unlink_child_ino),
            None => return Err(entity::Error::InternalError.into())
        }

        // 親attrのサイズを変更する
        match self.attr_mut() {
            Some(attr) => { attr.dec_size(parent); },
            None => return Err(entity::Error::InternalError.into())
        }

        // lookupcountをチェックし、
        // lookupcountが0ならば子attr,dataを削除する
        // 0ではないならばLookupCountayに記録する
        match self.lookup_count_mut() {
            Some(lookup_count) => match lookup_count.lookup_count(unlink_child_ino) {
                0 => {
                    // メモリ上から任意のinoを持つattr::Attrを削除する
                    match self.attr_mut() {
                        Some(attr) => { attr.del(unlink_child_ino); },
                        None => return Err(entity::Error::InternalError.into())
                    }

                    // メモリ上から任意のinoを持つdata::Dataを削除する
                    match self.data_mut() {
                        Some(data) => { data.del(unlink_child_ino); },
                        None => return Err(entity::Error::InternalError.into())
                    }
                },
                _ => {
                    lookup_count.delay(unlink_child_ino); 
                }
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // entry.yamlを更新する
        match self.entry() {
            Some(entry) => {
                self.file_repository.update_entry(parent, entry.entry(parent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // attr.yamlの更新
        match self.attr() {
            Some(attr) => {
                self.file_repository.update_attr(attr.attr(parent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }
        self.file_repository.del_attr(unlink_child_ino)?;

        // data.yamlを更新する
        self.file_repository.del_data(unlink_child_ino)?;

        Ok(())
    }

    fn forget(
        &mut self,
        ino: u64,
        nlookup: u64,
    ) -> Result<()> {
        if let Some(lookup_count) = self.lookup_count_mut() {
            match lookup_count.forget(ino, nlookup) {
                Some(num) => {
                    if num == 0 {
                        let file_type;

                        // メモリ上から任意のinoを持つattr::Attrを削除する
                        match self.attr_mut() {
                            Some(attr) => { 
                                file_type = attr.attr(ino).unwrap().file_type();

                                attr.del(ino);
                            },
                            None => return Err(entity::Error::InternalError.into())
                        }

                        // メモリ上から任意のinoを持つdata::Dataを削除する
                        match file_type {
                            attr::FileType::Directory => match self.entry_mut() {
                                Some(entry) => { entry.del(ino); },
                                None => return Err(entity::Error::InternalError.into())
                            },
                            attr::FileType::TextFile => match self.data_mut() {
                                Some(data) => { data.del(ino); },
                                None => return Err(entity::Error::InternalError.into())
                            }
                        }
                        
                    }
                },
                None => {}
            }
        }
        Ok(())
    }

    fn mkdir(
        &mut self,
        parent: u64,
        name: &OsStr,
        mode: u32,
    ) -> Result<attr::Attr> {
        let new_ino = self.new_ino();

        // attrの更新
        match self.attr_mut() {
            Some(attr) => {
                let name_string = match name.to_str() {
                    Some(name) => name.to_string(),
                    None => return Err(entity::Error::InternalError.into())
                };
                let new_attr = attr::Attr::new(
                    new_ino,
                    0,
                    name_string,
                    attr::FileType::Directory,
                    mode as u16,
                    1000,
                    1000,
                    attr::SystemTime::now(),
                    attr::SystemTime::now(),
                    attr::SystemTime::now(),
                    1
                );
                attr.inc_size(parent);
                attr.update_attr(new_attr);
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // entryの更新
        match self.entry_mut() {
            Some(entry) => {
                entry.insert_child_ino(parent, new_ino);
                entry.insert_entry(new_ino);
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // attr.yamlの更新
        match self.attr() {
            Some(attr) => {
                self.file_repository.update_attr(attr.attr(parent).unwrap())?;
                self.file_repository.update_attr(attr.attr(new_ino).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // entry.yamlの更新
        match self.entry() {
            Some(entry) => {
                self.file_repository.update_entry(new_ino, entry.entry(parent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }

        let attr_data = match self.attr() {
            Some(attr) => match attr.attr(new_ino) {
                Some(attr_data) => attr_data.clone(),
                None => return Err(entity::Error::InternalError.into())
            },
            None => return Err(entity::Error::InternalError.into())
        };


        match self.lookup_count_mut() {
            Some(lookup_count) => { lookup_count.update_lookupcount(new_ino); },
            None => return Err(entity::Error::InternalError.into())
        }
    
        Ok(attr_data)
    }

    fn rmdir(
        &mut self,
        parent: u64,
        name: &OsStr,
    ) -> Result<()> {
        // unlinkするディレクトリのino
        let child_ino = match self.child_ino_from_parent(parent, name) {
            Some(child_ino) => child_ino,
            None => return Err(entity::Error::InternalError.into())
        };

        // ディレクトリがからかどうかを確認
        match self.entry() {
            Some(entry) => {
                match entry.entry(child_ino) {
                    Some(entry) => {
                        if entry.len() != 0 {
                            return Err(entity::Error::InternalError.into());
                        }
                    },
                    None => return Err(entity::Error::InternalError.into())
                }
            },
            None => return Err(entity::Error::InternalError.into())
        };


        // 親entryから当該のエントリを削除する
        match self.entry_mut() {
            Some(entry) => entry.remove_child_ino(parent, child_ino),
            None => return Err(entity::Error::InternalError.into())
        }

        // 親attrのサイズを変更する
        match self.attr_mut() {
            Some(attr) => { attr.dec_size(parent); },
            None => return Err(entity::Error::InternalError.into())
        }

        // lookupcountを確認し、entryから削除
        match self.lookup_count_mut() {
            Some(lookup_count) => match lookup_count.lookup_count(child_ino) {
                0 => {
                    match self.attr_mut() {
                        Some(attr) => { attr.del(child_ino); },
                        None => return Err(entity::Error::InternalError.into())
                    }

                    match self.entry_mut() {
                        Some(entry) => entry.del(child_ino),
                        None => return Err(entity::Error::InternalError.into())
                    }
                },
                _ => {
                    lookup_count.delay(child_ino);
                }
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // entry.yamlからディレクトリを削除
        match self.entry() {
            Some(entry) => {
                self.file_repository.update_entry(parent, entry.entry(parent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }

        // attr.yamlの更新
        match self.attr() {
            Some(attr) => {
                self.file_repository.update_attr(attr.attr(parent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }
        self.file_repository.del_attr(child_ino)?;

        Ok(())
    }

    fn rename (
        &mut self,
        parent: u64,
        name: &OsStr,
        newparent: u64,
        newname: &OsStr,
    ) -> Result<()> {
        // 親ディレクトリが同じの場合
        // 名前だけ変更
        let file_type;

        let ino = match self.child_ino_from_parent(parent, name) {
            Some(ino) => ino,
            None => return Err(entity::Error::InternalError.into())
        };

        if parent == newparent {
            match self.attr_mut() {
                Some(attr) => {
                    let new_name_str = match newname.to_str() {
                        Some(name) => name,
                        None => return Err(entity::Error::InternalError.into())
                    };
                    attr.update_name(ino, new_name_str);
                },
                None => return Err(entity::Error::InternalError.into())
            }
        }

        // 変更先にファイル、ディレクトリがある場合は、自動で上書き
        // ディレクトリを上書きする際、変更先がからでない場合は、エラーを返却

        // 上書きが現状できてない
        // 同じファイルが2つある状態になる

        match self.child_ino_from_parent(newparent, newname) {
            Some(move_ino) => {
                // 上書きする場合
                // lookupを確認したあと
                // 上書きされるエントリのattrと、data or entryを削除
                match self.lookup_count_mut() {
                    Some(lookup_count) => match lookup_count.lookup_count(move_ino) {
                        0 => {
                            match self.attr_mut() {
                                // 削除するエントリのファイルタイプを取得
                                Some(attrs) => {
                                    match attrs.attr(move_ino) {
                                        Some(attr) => {
                                            file_type = attr.file_type();
                                        },
                                        None => return Err(entity::Error::InternalError.into())
                                    }
                                    // attrを削除
                                    attrs.del(move_ino);

                                    // file_typeをもとにdirectoryやtextfileを削除
                                    match file_type {
                                        attr::FileType::Directory => match self.entry_mut() {
                                            Some(entry) => {
                                                entry.del(move_ino)
                                            },
                                            None => return Err(entity::Error::InternalError.into())
                                        },
                                        attr::FileType::TextFile => match self.data_mut() {
                                            Some(data) => { data.del(move_ino); },
                                            None => return Err(entity::Error::InternalError.into())
                                        }
                                    }
                                },
                                None => return Err(entity::Error::InternalError.into())
                            }
                            // entryを削除

                            match self.entry_mut() {
                                Some(entry) => {
                                    entry.remove_child_ino(newparent, ino);
                                    entry.mov(ino, parent, newparent);
                                },
                                None => return Err(entity::Error::InternalError.into())
                            }

                            self.file_repository.del_attr(move_ino)?;
                        },
                        _ => {
                            lookup_count.delay(move_ino);
                            match self.entry_mut() {
                                Some(entry) => {
                                    entry.remove_child_ino(newparent, move_ino);
                                    entry.mov(ino, parent, newparent);
                                },
                                None => return Err(entity::Error::InternalError.into())
                            }

                            self.file_repository.del_attr(move_ino)?;
                        }
                    },
                    None => return Err(entity::Error::InternalError.into())
                }
            },
            None => {
                // 上書きせずにそのまま移動
                match self.entry_mut() {
                    Some(entry) => {
                        entry.mov(ino, parent, newparent);
                    },
                    None => return Err(entity::Error::InternalError.into())
                }
            }
        }

        match self.attr() {
            Some(attr) => {
                self.file_repository.update_attr(attr.attr(parent).unwrap())?;
                self.file_repository.update_attr(attr.attr(newparent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }

        match self.entry() {
            Some(entry) => {
                self.file_repository.update_entry(parent, entry.entry(parent).unwrap())?;
                self.file_repository.update_entry(newparent, entry.entry(newparent).unwrap())?;
            },
            None => return Err(entity::Error::InternalError.into())
        }

        Ok(())
    }

    fn new_ino(&mut self) -> u64 {
        let next_ino = match self.next_ino {
            Some(next_ino) => {
                self.next_ino = Some(next_ino + 1);
                next_ino + 1
            },
            None => return 0
        };
        next_ino
    }
}

impl<F: repository::File>  UsecaseStruct<F> {
    fn entry(&self) -> Option<&entry::EntriesStruct> {
        match &self.entry {
            Some(entry) => Some(entry),
            None => None
        }
    }

    fn entry_mut(&mut self) -> Option<&mut entry::EntriesStruct> {
        match &mut self.entry {
            Some(entry) => Some(entry),
            None => None
        }
    }

    fn attr(&self) -> Option<&attr::AttrsStruct> {
        match &self.attr {
            Some(attr) => Some(attr),
            None => None
        }
    }

    fn attr_mut(&mut self) -> Option<&mut attr::AttrsStruct> {
        match &mut self.attr {
            Some(attr) => Some(attr),
            None => None
        }
    }

    fn data(&self) -> Option<&data::AllDataStruct> {
        match &self.data {
            Some(data) => Some(data),
            None => None
        }
    }

    fn data_mut(&mut self) -> Option<&mut data::AllDataStruct> {
        match &mut self.data {
            Some(data) => Some(data),
            None => None
        }
    }

    fn lookup_count(&self) -> Option<&lookup_count::LookupCount> {
        match &self.lookup_count {
            Some(lookup_count) => Some(lookup_count),
            None => None
        }
    }

    fn lookup_count_mut(&mut self) -> Option<&mut lookup_count::LookupCount> {
        match &mut self.lookup_count {
            Some(lookup_count) => Some(lookup_count),
            None => None
        }
    }

    fn child_ino_from_parent(&self, parent_ino: u64, name: &OsStr) -> Option<u64> {
        let entries = match self.entry() {
            Some(entries) => match entries.entry(parent_ino) {
                Some(entry) => entry,
                None => return None
            },
            None => return None
        };

        // 親ディレクトリのエントリからnameの名前を持つ子どをも探索する
        for entry in entries.iter() {
            let child_ino = entry.child_ino();
            let child_attr = match self.attr() {
                Some(attr) => match attr.attr(child_ino) {
                    Some(child_attr) => child_attr,
                    None => return None
                },
                None => return None
            };
            let file_name = match name.to_str() {
                Some(file_name) => file_name,
                None => return None
            };

            if child_attr.name == file_name {
                return Some(child_ino);
            }
        };

        None
    }
}


// この関数テストコード欲しい
// バグが多い気がする
fn merge_str(offset: u64, data: &str, old_str: &str) -> Result<String> {
    let mut new_string_len = offset + data.len() as u64;
    if new_string_len < old_str.len() as u64 {
        new_string_len = old_str.len() as u64;
    }
    let mut new_string_vec = Vec::with_capacity(new_string_len as usize);

    let data_bytes = data.as_bytes();
    let old_str_bytes = old_str.as_bytes();

    for _ in 0..new_string_len {
        new_string_vec.push(0);
    }

    
    for (i, &data) in old_str_bytes.iter().enumerate() {
        new_string_vec[i] = data;
    }

    for (i, &data) in data_bytes.iter().enumerate() {
        new_string_vec[i + offset as usize] = data;
    }

    match String::from_utf8(new_string_vec) {
        Ok(string) => Ok(string),
        Err(e) => Err(e.into())
    }
}

fn begger_data(data: &str, size: u64) -> String {
    let mut new_string_vec = Vec::with_capacity(size as usize);
    let data_bytes = data.as_bytes();

    for &data in data_bytes.iter() {
        new_string_vec.push(data);
    }
    for _ in data.len()..(size as usize) {
        new_string_vec.push(0);
    }

    match String::from_utf8(new_string_vec) {
        Ok(string) => string,
        Err(e) => String::new()
    }
}

fn smaller_data(data: &str,  size: u64) -> String {

    let mut new_string_vec = Vec::with_capacity(size as usize);
    let data_bytes = data.as_bytes();

    for (i, &data) in data_bytes.iter().enumerate() {
        if size  == i as u64 {
            break;
        }
        new_string_vec.push(data);
    }

    match String::from_utf8(new_string_vec) {
        Ok(string) => string,
        Err(e) => String::new()
    }
}