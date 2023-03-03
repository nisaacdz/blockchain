use serde::Serialize;

use crate::{
    chain::{Block, PublishedBlock, Record, SignedRecord},
    errs::CustomErrs,
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
    fn insert_block(&self, block: &Block<T>) -> Result<QueryRange, CustomErrs> {
        let begin = self.next_stamp();

        let end = begin + block.size() - 1;

        let block_position = QueryRange::new(begin, end);

        let mut count = begin;

        for record in block.signed_records.iter() {
            self.insert_row(record, count)?;
            count += 1;
        }

        Ok(block_position)
    }
    fn insert_row(&self, record: &SignedRecord<T>, stamp: i64) -> Result<(), CustomErrs>;
    fn insert_hash(&self, published_block: PublishedBlock) -> Result<(), CustomErrs>;
    fn next_stamp(&self) -> i64;
}

///
///
///
///
///
///
use std::collections::HashSet;

pub trait DatabaseInsertable {
    fn get_name() -> &'static str;

    fn columns() -> &'static [&'static str];

    fn len(&self) -> i64;
}


pub trait Database2 {
    /// Creates a new table with the given name, if table doesn't already exist.
    ///
    /// `Ok(())` value indicates success whiles `CustomErrs` indicate different failures
    fn create_table<T: DatabaseInsertable>(&mut self) -> Result<(), CustomErrs>;

    /// Returns `None` if the table is not present in the database
    /// and `Some(number of rows)` if the table exists
    fn size_of_table<T: DatabaseInsertable>(&self) -> Option<i64> {
        if self.table_exists::<T>() {
            Some(self.len(&T::get_name()))
        } else {
            None
        }
    }

    /// Do not use, may fail
    fn len(&self, table_name: &str) -> i64;

    /// Checks if the given type has a table in the database
    fn table_exists<T: DatabaseInsertable>(&self) -> bool {
        self.get_tables().contains(T::get_name())
    }

    /// Returns a mutable reference to the set of table names in the database
    fn get_tables_mut(&mut self) -> &mut HashSet<String>;

    ///Inserts a new table for the given type it doesn't already exist
    ///
    ///It attempts to insert the table without checking if it is already present in `get_tables()`
    ///
    /// Use it if `get_tables()` might be misleading
    fn insert_table<T: DatabaseInsertable>(&mut self) -> Result<(), CustomErrs> {
        self.get_tables_mut().insert(T::get_name().to_owned());
        self.create_table::<T>()
    }

    /// Returns an immutable reference to the tables in the database
    fn get_tables(&self) -> &HashSet<String>;

    /// Inserts the given Item into its table in the database
    fn insert<T: DatabaseInsertable + Copy + IntoIterator<Item = Vec<String>>>(
        &mut self,
        item: &T,
    ) -> Result<QueryRange, CustomErrs> {
        if self.table_exists::<T>() {
            self.insert_table_exists(item)
        } else {
            self.insert_table::<T>()?;
            self.insert_table_exists(item)
        }
    }
    /// Use with caution. Recommended `insert()`
    ///
    /// This inserts item into the table for T given that the table for T exists in the database
    fn insert_table_exists<T: DatabaseInsertable + Copy + IntoIterator<Item = Vec<String>>>(
        &self,
        item: &T,
    ) -> Result<QueryRange, CustomErrs> {
        let begin = self.len(T::get_name());
        let end = begin + item.len() - 1;

        let mut iter_vec = item.into_iter();

        for _ in begin..=end {
            let vec = iter_vec.next().unwrap();
            self.insert_row::<T>(&vec)?
        }

        Ok(QueryRange { begin, end })
    }
    /// Inserts an insertable object into the database
    fn insert_row<T: DatabaseInsertable>(&self, columns: &[String]) -> Result<(), CustomErrs>;
}
