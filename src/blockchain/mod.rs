use ring::signature::Signature;
use serde::{Deserialize, Serialize};

use crate::{
    errs::Errs,
    gen,
    gen::Hash,
    io::{Database, TimeStamp},
};

pub struct SignedRecord<T: Record> {
    pub public_key: Vec<u8>,
    pub signature: Signature,
    pub record: T,
}

impl<T: Record> SignedRecord<T> {
    fn verify(&self) -> Result<(), Errs> {
        let msg = bincode::serialize(&self.record).unwrap();
        gen::verify_signature(&self.public_key, &msg, self.signature)
    }

    fn is_valid(&self) -> bool {
        self.verify().is_ok()
    }
}

pub trait Record
where
    Self: Clone + Sized + Serialize + for<'a> Deserialize<'a>,
{
    fn hash(&self) -> Hash {
        gen::encrypt(self)
    }

    fn sign(&self, private_key: &[u8], public_key: &[u8]) -> Result<SignedRecord<Self>, Errs> {
        let msg = bincode::serialize(self).unwrap();
        let signature = gen::sign(&msg, private_key)?;
        let public_key = public_key.to_vec();
        Ok(SignedRecord {
            public_key,
            signature,
            record: self.clone(),
        })
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
        Self { db }
    }

    fn append<R: Record>(&self, block: Block<R>) -> TimeStamp {
        self.db.insert(block)
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
