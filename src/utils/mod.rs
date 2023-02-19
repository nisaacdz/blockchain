use serde::{Deserialize, Serialize};

use crate::blockchain::Record;

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
impl Record for Transaction {}
