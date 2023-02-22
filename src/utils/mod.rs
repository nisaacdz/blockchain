use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::{
    block,
    blockchain::{Block, Record, SignedRecord},
    errs::CustomErrs,
    gen::Hash,
    io::{Database, QueryRange},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    src: String,
    dst: String,
    amount: String,
}

pub trait Entity<T: Record> {
    fn public_key(&self) -> &[u8];
    fn sign_record(&self, record: T, pkey: &[u8]) -> Result<SignedRecord<T>, CustomErrs> {
        record.sign(pkey, &self.public_key())
    }
}

pub struct Miner<T: Record> {
    block: Block<T>,
}

impl<T: Record> Miner<T> {
    ///Initializes a new instance of Miner with an empty block

    pub fn new() -> Self {
        Self { block: block![] }
    }

    pub fn get_block(&self) -> &Block<T> {
        &self.block
    }

    pub fn add_to_block(&mut self, record: SignedRecord<T>) {
        self.block.append(record);
    }

    /// Returns `Ok(())` if record is valid or an `CustomErrs` variant
    ///matching the type of failure
    pub fn verify_record(&self, record: SignedRecord<T>) -> Result<(), CustomErrs> {
        record.verify()
    }
}

impl Transaction {
    pub fn new(src: &str, dst: &str, amount: &str) -> Self {
        Self {
            src: src.to_owned(),
            dst: dst.to_owned(),
            amount: amount.to_owned(),
        }
    }
}

pub struct SqliteDB {
    con: Connection,
}

impl SqliteDB {
    pub fn open(path: &str) -> Result<Self, CustomErrs> {
        Ok(Self {
            con: Connection::open(path)
                .map_err(|_| CustomErrs::CannotEstablishDatabaseConnection)?,
        })
    }

    fn add_record<R: Record>(&self, record: SignedRecord<R>, stamp: i64) -> Result<(), CustomErrs> {
        let rstring: String = serde_json::to_string(record.get_record()).unwrap();
        let signature = serde_json::to_string(record.get_signature().as_ref()).unwrap();
        let id = serde_json::to_string(record.get_signer()).unwrap();

        self.con
            .execute(
                "INSERT INTO records (Position, Record, Identity, Signature) VALUES (?, ?, ?, ?)",
                params![stamp, rstring, id, signature],
            )
            .map_err(|_| CustomErrs::CouldNotInsertRecordsIntoDatabase)?;

        Ok(())
    }
}

impl Record for Transaction {}

impl Database<Transaction> for SqliteDB {
    fn establish_connection(&self) -> Result<(), CustomErrs> {
        Ok(())
    }

    fn next_stamp(&self) -> i64 {
        let mut stmt = self.con.prepare(&"SELECT COUNT(*) FROM records").unwrap();
        let count = stmt.query_row([], |row| row.get(0)).unwrap();
        count
    }

    fn insert_row(&self, record: SignedRecord<Transaction>, stamp: i64) -> Result<(), CustomErrs> {
        self.add_record(record, stamp)
    }

    fn insert_hash(&self, hash: &Hash, block_position: QueryRange) -> Result<(), CustomErrs> {
        let hash = hash.to_string();
        let block_position = serde_json::to_string(&block_position).unwrap();

        self.con
            .execute(
                "INSERT INTO hash (Hash, BlockPosition) VALUES (?, ?)",
                params![hash, block_position],
            )
            .map_err(|_| CustomErrs::CouldNotInsertHashIntoDatabase)?;

        Ok(())
    }
}
