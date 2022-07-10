#[macro_use]
use lazy_static::lazy_static;
use std::{env, fs};
use std::path::PathBuf;
use leveldb::{
    database::Database,
    iterator::Iterable,
    kv::KV,
    options::{Options, WriteOptions, ReadOptions}
};

lazy_static!(
    pub static ref D: Database<i32> = {
        let mut options = Options::new();
        options.create_if_missing = true;

        let database = match Database::open(PathBuf::from(".maelstrom").as_path(), options) {
            Ok(db) => {
                db
            },
            Err(e) => { panic!("failed to open database: {:?}", e) }
        };
        //println!("Previous block hash : {:?}", database.iter(ReadOptions::new()).last().unwrap().1);
        return database;
    };
);

pub fn open_db(path: PathBuf) -> Database<i32> {
    let mut options = Options::new();
    options.create_if_missing = true;

    let database = match Database::open(path.as_path(), options) {
        Ok(db) => { return db },
        Err(e) => { panic!("failed to open database: {:?}", e) }
    };
    return database;
}

pub fn write_db(database: &Database<i32>, key: i32, value: &[u8]) {
    let write_opts = WriteOptions::new();
    match database.put(write_opts, key, value) {
        Ok(_) => { () },
        Err(e) => { panic!("failed to write to database: {:?}", e) }
    };
}

pub fn read_db(database: &Database<i32>, key: i32) -> Vec<u8> {
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, key);
    match res {
        Ok(data) => {
            println!("{:?}", data);
            return data.unwrap();
        }
        Err(e) => { panic!("failed reading data: {:?}", e) }
    }
}

pub fn get_last_db_value(database: &Database<i32>) -> Vec<u8> {
    return database.iter(ReadOptions::new()).last().unwrap().1;
}

pub fn create_blockchain_dir() -> PathBuf {
    let mut path = env::current_dir().unwrap();
    path.push(".maelstrom");
    fs::create_dir_all(path.as_path()).unwrap();
    return path;
}