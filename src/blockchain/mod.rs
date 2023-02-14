use rusqlite::{params, Connection};
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

use crate::{hash, hash::Hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    src: String,
    dst: String,
    amount: String,
}

impl Transaction {
    pub fn new(src: &str, dst: &str, amount: &str) -> Self {
        Self { src: src.to_owned(), dst: dst.to_owned(), amount: amount.to_owned() }
    }
}

impl Record for Transaction {}

pub trait Record
where
    Self: Sized + Serialize + for<'a> Deserialize<'a>,
{
    fn hash(&self) -> Hash {
        hash::encrypt(self)
    }
}

pub struct TimeStamp {
    pub value: u128,
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:0width$}", self.value, width = 128))
    }
}

impl Add<usize> for TimeStamp {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            value: self.value + rhs as u128,
        }
    }
}

impl Sub<usize> for TimeStamp {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self {
            value: self.value - rhs as u128,
        }
    }
}

impl TimeStamp {
    pub fn get_value(&self) -> i64 {
        self.value as i64
    }
}

#[derive(Debug)]
pub struct Block<R: Record> {
    pub records: Vec<R>,
}

#[macro_export]
macro_rules! block {
    ($($record:expr),*) => {
        {
            let records = vec![$($record),*];
            blockchain::blockchain::Block { records }
        }
    }
}

impl<R: Record> Block<R> {}

pub struct BlockChain {
    conn: Connection,
}

impl BlockChain {
    pub fn new() -> Self {
        //keys in the block chain are the timestamps, values is dbg!(block)
        Self {
            conn: Connection::open(r"db\mydatabase.db").unwrap(),
        }
    }

    pub fn append<R: Record>(&self, block: Block<R>) -> TimeStamp {
        let value: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM mytable", [], |row| row.get(0))
            .unwrap();

        let records = serde_json::to_string(&block.records).unwrap();

        self.conn
            .execute(
                "INSERT INTO mytable  (timestamp, blocks) VALUES (?, ?)",
                params![value, records],
            )
            .unwrap();

        TimeStamp { value: value as _ }
    }

    pub fn get_block<R: Record>(&self, timestamp: TimeStamp) -> Block<R> {
        let res: String = self.conn.query_row(
            "SELECT blocks FROM mytable WHERE timestamp = ?1",
            params![timestamp.get_value()],
            |row| row.get(0),
        )
        .unwrap();
        let vec: Vec<R> = serde_json::from_str(&res).unwrap();
        Block {
            records: vec,
        }
    }
}
