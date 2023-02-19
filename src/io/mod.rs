use rusqlite::Connection;

use crate::blockchain::{Block, Record, SignedRecord};

pub struct TimeStamp {
    begin: usize,
    end: usize,
}

pub struct Database {
    path: String,
}

impl Database {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_owned(),
        }
    }
    pub fn open(&self) -> Connection {
        // r"db\mydatabase.db"
        Connection::open(&self.path).unwrap()
    }

    pub fn insert<T: Record>(&self, block: Block<T>) -> TimeStamp {
        unimplemented!()
    }
}
