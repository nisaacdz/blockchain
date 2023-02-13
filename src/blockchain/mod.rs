use std::cell::RefCell;

use serde::Serialize;

use crate::{hash, hash::Hash};

#[derive(Debug, Serialize)]
pub struct Transaction {
    
}

impl Record for Transaction {}

pub trait Record where Self: Sized + Serialize {
    fn hash(&self) -> Hash {
        hash::encrypt(self)
    }
}

struct Block<Z> where Z: Record {
    // Ledger
    records: Vec<RefCell<Z>>,
}