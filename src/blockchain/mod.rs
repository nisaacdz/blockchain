use ring::signature::Signature;
use serde::{Deserialize, Serialize};

use crate::{gen, gen::Hash, io::{TimeStamp, Database}, errs::Errs};

pub struct SignedRecord<T: Record> {
    pub signature: Signature,
    pub record: T,
}

impl<T: Record> SignedRecord<T> {
    fn is_valid(&self) -> bool {
        todo!()
    }
}

pub trait Record
where
    Self: Sized + Serialize + for<'a> Deserialize<'a>,
{
    fn hash(&self) -> Hash {
        gen::encrypt(self)
    }

    fn sign(&self, key: &[u8]) -> Result<SignedRecord<Self>, Errs> {
        unimplemented!()
    }
}

pub struct Block<R: Record> {
    pub signed_records: Vec<SignedRecord<R>>,
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

////
/// 
/// 
/////
/// 
/// 
/// 
/// 

pub struct BlockChain {
    db: Database,
}
impl BlockChain {
    pub fn open(db: Database) -> Self {
        //keys in the block chain are the timestamps, values is dbg!(block)
        Self {
            db
        }
    }

    fn append<R: Record>(&self, block: Block<R>) -> TimeStamp {
        let Block { signed_records } = block;
        
        unimplemented!()
    }

    //
    pub fn push<R: Record>(&self, block: Block<R>) -> Result<TimeStamp, Errs> {
        if block.signed_records.iter().all(|r| r.is_valid()) {
            Ok(self.append(block))
        } else {
            Err(Errs::InvalidBlock)
        }        
    }

    pub fn get_block<R: Record>(&self, _timestamp: TimeStamp) -> Block<R> {
        unimplemented!()
    }
}
