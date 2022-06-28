//src/main.rs
use std::{env, fs};
use leveldb::{
    database::Database,
    iterator::Iterable,
    kv::KV,
    options::{Options, WriteOptions, ReadOptions}
};

pub fn main() {
    let mut dir = env::current_dir().unwrap();
    dir.push("msblk");

    let path_buf = dir.clone();
    fs::create_dir_all(dir).unwrap();

    let path = path_buf.as_path();
    let mut options = Options::new();
    options.create_if_missing = true;

    //Create database
    let database = match Database::open(path, options) {
        Ok(db) => { db },
        Err(e) => { panic!("failed to open database: {:?}", e) }
    };

    //Write database
    let write_opts = WriteOptions::new();
    match database.put(write_opts, 1, &[1]) {
        Ok(_) => { () },
        Err(e) => { panic!("failed to write to database: {:?}", e) }
    };

    //Read database
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 1);

    match res {
        Ok(data) => {
            assert!(data.is_some());
            assert_eq!(data, Some(vec![1]));
        }
        Err(e) => { panic!("failed reading data: {:?}", e) }
    }

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);
    let entry = iter.next();
    assert_eq!(
        entry,
        Some((1, vec![1]))
    );
}
