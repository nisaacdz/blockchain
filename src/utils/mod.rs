use std::collections::HashSet;

use rusqlite::{params, Connection, ToSql};
use serde::{Deserialize, Serialize};

use crate::{
    block,
    blockchain::{Block, FeedBack, PublishedBlock, Record, SignedRecord},
    errs::CustomErrs,
    io::{Database, Database2, DatabaseInsertable},
    node::NodeId,
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
    fn receive_broadcast(&self, block: &FeedBack<T>, from_node: NodeId);
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

    fn add_record<R: Record>(
        &self,
        record: &SignedRecord<R>,
        stamp: i64,
    ) -> Result<(), CustomErrs> {
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

    fn insert_row(&self, record: &SignedRecord<Transaction>, stamp: i64) -> Result<(), CustomErrs> {
        self.add_record(record, stamp)
    }

    fn insert_hash(&self, published_block: PublishedBlock) -> Result<(), CustomErrs> {
        let s = published_block.into_iter().next().unwrap();

        self.con
            .execute(
                "INSERT INTO hash (Hash, BlockPosition) VALUES (?, ?)",
                params![s[0], s[1]],
            )
            .map_err(|_| CustomErrs::CouldNotInsertHashIntoDatabase)?;

        Ok(())
    }
}

pub struct SqliteDB2 {
    tables: HashSet<String>,
    connection: Connection,
}
impl SqliteDB2 {
    pub fn new(path: &str) -> Self {
        Self {
            tables: HashSet::new(),
            connection: Connection::open(path).unwrap(),
        }
    }
}

impl Database2 for SqliteDB2 {
    fn create_table<T: DatabaseInsertable>(&mut self) -> Result<(), CustomErrs> {
        let table_name = T::get_name();
        let columns = T::columns();
        let mut column_defs = Vec::new();

        column_defs.push("Position INTEGER PRIMARY KEY AUTOINCREMENT".to_owned());

        columns
            .iter()
            .for_each(|v| column_defs.push(format!("{} TEXT", v)));

        let create_stmt = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name,
            column_defs.join(", ")
        );

        self.connection
            .execute(&create_stmt, [])
            .map_err(|_| CustomErrs::CannotCreateSuchTable)?;

        self.tables.insert(table_name.to_string());

        Ok(())
    }

    fn len(&self, table_name: &str) -> i64 {
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT COUNT(*) FROM {}", table_name))
            .unwrap();
        let count = stmt.query_row([], |row| row.get(0)).unwrap();
        count
    }

    fn get_tables(&self) -> &HashSet<String> {
        &self.tables
    }

    fn insert_row<T: DatabaseInsertable>(&self, items: &[String]) -> Result<(), CustomErrs> {
        let table_name = T::get_name();
        let columns = T::columns();
        let num_columns = columns.len();

        let placeholders = vec!["?"; num_columns].join(", ");
        let column_names = vec![columns[0..].join(", ")].join(", ");
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, column_names, placeholders
        );

        let mut stmt = self.connection.prepare(&sql).unwrap();
        let params = rusqlite::params_from_iter(items.iter().map(|x| x as &dyn ToSql));

        stmt.execute(params).unwrap();

        Ok(())
    }

    fn get_tables_mut(&mut self) -> &mut HashSet<String> {
        &mut self.tables
    }
}
