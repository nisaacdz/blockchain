use crate::{
    blockchain::{Block, Record, SignedRecord},
    errs::Errs,
};

#[derive(Debug, Copy, Clone)]
pub struct TimeStamp {
    pub begin: i64,
    pub end: i64,
}

impl TimeStamp {
    fn new(begin: i64, end: i64) -> Self {
        Self { begin, end }
    }
}

pub trait Database<T>
where
    T: Record,
{
    fn establish_connection(&self) -> Result<(), Errs>;
    fn insert_block(&self, block: Block<T>) -> Result<TimeStamp, Errs> {
        let begin = self.next_stamp();

        let end = begin + block.size() - 1;

        let timestamp = TimeStamp::new(begin, end);

        let mut count = begin;

        for record in block.signed_records {
            self.insert_row(record, count)
                .map_err(|_| Errs::CouldNotInsertIntoDatabase)?;
            count += 1;
        }

        Ok(timestamp)
    }
    fn insert_row(&self, record: SignedRecord<T>, stamp: i64) -> Result<(), Errs>;
    fn next_stamp(&self) -> i64;
}
