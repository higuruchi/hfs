# hfs

## 概要・機能

fuseの練習を目的としたアプリケーション

yamlファイルをディレクトリ構造に見立て、ファイルやディレクトリのCRUD操作を行う。

## 利用環境

- rust
- fuse

## 利用方法

```bash
$ hfs --config-path /path/to/config --mountpoint /path/to/mountpoint
```

### configファイルの記述方法

```yaml
# 親inodeと子inodeの関係を記述しているentry.yamlへのパス
entry: /path/to/entry.yaml

# ファイルの内容を記述しているdata.yamlへのパス
data: /path/to/entry.yaml

# ファイルやディレクトリの属性の情報を記述しているattr.yamlへのパス
attr: /path/to/attr.yaml
```

```yaml
# ファイルやディレクトリの属性の情報を記述しているattr.yamlの記述方法

- ino: 1
  name: root
  file-type: 0
  size: 2

- ino: 2
  name: file1
  file-type: 1
  size: 20

- ino: 3
  name: directory1
  file-type: 0
  size: 1

- ino: 4
  name: file2
  file-type: 1
  size: 0
```

```yaml
# 親inodeと子inodeの関係を記述しているentry.yamlの記述方法

- ino: 1
  files:
    - 2
    - 3

- ino: 3
  files:
    - 4
```

```yaml
# ファイルの内容を記述しているdata.yamlの記述方法

- ino: 2
  data: "this is content of file1"

- ino: 4
  data: ""
```

## インストール方法

```bash
$ git clone https://github.com/higuruchi/hfs.git
$ cd hfs
$ make all
$ ./target/debug/hfs --config-path ./tests/config/image.yaml --mountpoint ./mountpoint &
```

## 実装済みのインタフェース

- init
- lookup
- readdir
- read
- write
- setattr
- create
- unlink
- forget
- mkdir
- rmdir
- rename
