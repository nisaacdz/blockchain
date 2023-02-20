use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::{blockchain::{Record, SignedRecord}, errs::Errs, io::Database};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    src: String,
    dst: String,
    amount: String,
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
    pub fn open(path: &str) -> Result<Self, Errs> {
        Ok(Self { con: Connection::open(path).map_err(|_| Errs::CannotEstablishDatabaseConnection)? })
    }

    pub fn create_table(&self) -> Result<(), Errs> {
        self.con.execute(
            "CREATE TABLE IF NOT EXISTS transactionchain (
                TimeStamp INTEGER PRIMARY KEY,
                Record TEXT,
                Identity TEXT,
                Signature TEXT
            )",
            params![],
        ).unwrap();
    
        Ok(())
    }

    fn add_record<R: Record>(&self, record: SignedRecord<R>, stamp: i64) -> Result<(), Errs> {
        let rstring: String = serde_json::to_string(record.get_record()).unwrap();
        let signature = serde_json::to_string(record.get_signature().as_ref()).unwrap();
        let id = serde_json::to_string(record.get_signer()).unwrap();

        self.con.execute(
            "INSERT INTO transactionchain (TimeStamp, Record, Identity, Signature) VALUES (?, ?, ?, ?)",
            params![stamp, rstring, id, signature],
        ).map_err(|_| Errs::CouldNotInsertIntoDatabase)?;
        
        Ok(())
    }
}


impl Record for Transaction {}

impl Database<Transaction> for SqliteDB {
    fn establish_connection(&self) -> Result<(), Errs> {
        Ok(())
    }

    fn next_stamp(&self) -> i64 {
        let mut stmt = self.con.prepare(&"SELECT COUNT(*) FROM transactionchain").unwrap();
        let count = stmt.query_row([], |row| row.get(0)).unwrap();
        count
    }

    fn insert_row(&self, record: SignedRecord<Transaction>, stamp: i64) -> Result<(), Errs> {
        self.add_record(record, stamp)
    }

}

