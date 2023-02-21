use serde::Serialize;

use crate::{
    blockchain::{Block, Record, SignedRecord},
    errs::CustomErrs,
    gen::Hash,
};

#[derive(Serialize, Debug, Copy, Clone)]
pub struct QueryRange {
    pub begin: i64,
    pub end: i64,
}

impl QueryRange {
    fn new(begin: i64, end: i64) -> Self {
        Self { begin, end }
    }
}

pub trait Database<T>
where
    T: Record,
{
    fn establish_connection(&self) -> Result<(), CustomErrs>;
    fn insert_block(&self, block: Block<T>) -> Result<QueryRange, CustomErrs> {
        let begin = self.next_stamp();

        let end = begin + block.size() - 1;

        let block_position = QueryRange::new(begin, end);

        let mut count = begin;

        for record in block.signed_records {
            self.insert_row(record, count)?;
            count += 1;
        }

        Ok(block_position)
    }
    fn insert_row(&self, record: SignedRecord<T>, stamp: i64) -> Result<(), CustomErrs>;
    fn insert_hash(&self, hash: &Hash, block_position: QueryRange) -> Result<(), CustomErrs>;
    fn next_stamp(&self) -> i64;
}
