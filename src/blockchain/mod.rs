use std::marker::PhantomData;

use ed25519_dalek::Signature;
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

    pub fn get_signature(&self) -> &Signature {
        &self.signature
    }

    pub fn get_record(&self) -> &T {
        &self.record
    }

    pub fn get_signer(&self) -> &Vec<u8> {
        &self.public_key
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
    ($($signed_records:expr),*) => {
        {
            let signed_records = vec![$($signed_records),*];
            blockchain::blockchain::Block { signed_records }
        }
    }
}

impl<R: Record> Block<R> {
    pub fn size(&self) -> i64 {
        self.signed_records.len() as i64
    }
}

///
///
///
///
///
///
///
///

pub struct BlockChain<D: Database<R>, R: Record> {
    database: D,
    phantom_r: PhantomData<R>,
}

impl<D: Database<R>, R: Record> BlockChain<D, R> {
    pub fn open(database: D) -> Self {
        //keys in the block chain are the timestamps, values is dbg!(block)
        Self {
            database,
            phantom_r: PhantomData,
        }
    }

    fn append(&self, block: Block<R>) -> Result<TimeStamp, Errs> {
        self.database.insert_block(block)
    }

    pub fn push(&self, block: Block<R>) -> Result<TimeStamp, Errs> {
        if block.signed_records.iter().all(|r| r.is_valid()) {
            self.append(block)
        } else {
            Err(Errs::InvalidBlock)
        }
    }

    pub fn get_block(&self, _timestamp: TimeStamp) -> Block<R> {
        unimplemented!()
    }
}
