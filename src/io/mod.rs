use serde::Serialize;

use crate::{
    blockchain::{Block, Record, SignedRecord},
    errs::Errs,
    gen::Hash,
};

#[derive(Serialize, Debug, Copy, Clone)]
pub struct BlockPosition {
    pub begin: i64,
    pub end: i64,
}

impl BlockPosition {
    fn new(begin: i64, end: i64) -> Self {
        Self { begin, end }
    }
}

pub trait Database<T>
where
    T: Record,
{
    fn establish_connection(&self) -> Result<(), Errs>;
    fn insert_block(&self, block: Block<T>) -> Result<BlockPosition, Errs> {
        let begin = self.next_stamp();

        let end = begin + block.size() - 1;

        let block_position = BlockPosition::new(begin, end);

        let mut count = begin;

        for record in block.signed_records {
            self.insert_row(record, count)?;
            count += 1;
        }

        Ok(block_position)
    }
    fn insert_row(&self, record: SignedRecord<T>, stamp: i64) -> Result<(), Errs>;
    fn insert_hash(&self, hash: Hash, block_position: BlockPosition) -> Result<(), Errs>;
    fn next_stamp(&self) -> i64;
}
