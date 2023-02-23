use std::{fmt::Debug, marker::PhantomData};

use gen::Signature;
use serde::{Deserialize, Serialize};

use crate::{
    errs::CustomErrs,
    gen,
    gen::Hash,
    io::{Database, QueryRange},
};

#[derive(Debug, Serialize, Clone)]
pub struct SignedRecord<T: Record> {
    pub public_key: Vec<u8>,
    pub signature: Signature,
    pub record: T,
}

impl<T: Record> SignedRecord<T> {
    pub fn verify(&self) -> Result<(), CustomErrs> {
        let msg = bincode::serialize(&self.record).unwrap();
        gen::verify_signature(&self.public_key, &msg, &self.signature)
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
    fn sign(
        &self,
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<SignedRecord<Self>, CustomErrs> {
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

#[derive(Debug, Serialize, Clone)]
pub struct Block<R: Record> {
    pub signed_records: Vec<SignedRecord<R>>,
}
#[macro_export]
macro_rules! block {
    ($($signed_records:expr),*) => {
        {
            let signed_records = vec![$($signed_records),*];
            Block { signed_records }
        }
    }
}

impl<R: Record> Block<R> {
    pub fn append(&mut self, signed_record: SignedRecord<R>) {
        self.signed_records.push(signed_record)
    }
    pub fn size(&self) -> i64 {
        self.signed_records.len() as i64
    }

    pub fn verify(&self) -> Result<VerifiedBlock<R>, CustomErrs> {
        if self.signed_records.iter().all(|r| r.is_valid()) {
            let hash = gen::encrypt(&self);
            Ok(VerifiedBlock {
                hash,
                block: self.clone(),
            })
        } else {
            Err(CustomErrs::InvalidBlock)
        }
    }
}

pub struct VerifiedBlock<R: Record> {
    hash: Hash,
    block: Block<R>,
}

impl<R: Record> VerifiedBlock<R> {
    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }

    pub fn get_block(&self) -> &Block<R> {
        &self.block
    }
}

#[derive(Debug)]
pub struct FeedBack<R: Record> {
    pub block_position: QueryRange,
    pub hash: Hash,
    pub block: Block<R>,
}

impl<R: Record> FeedBack<R> {
    pub fn get_block(&self) -> &Block<R> {
        &self.block
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
        Self {
            database,
            phantom_r: PhantomData,
        }
    }

    fn append(&self, block: &Block<R>) -> Result<QueryRange, CustomErrs> {
        self.database.insert_block(block)
    }

    fn record(&self, hash: &Hash, block_position: QueryRange) -> Result<(), CustomErrs> {
        self.database.insert_hash(hash, block_position)
    }

    pub fn push(&self, block: &Block<R>) -> Result<FeedBack<R>, CustomErrs> {
        if block.signed_records.is_empty() {
            return Err(CustomErrs::EmptyBlocksNotAllowed);
        }

        let VerifiedBlock { hash, block } = block.verify()?;
        let block_position = self.append(&block)?;
        self.record(&hash, block_position)?;
        Ok(FeedBack {
            hash,
            block: block.clone(),
            block_position,
        })
    }

    pub fn get_records(&self, _block_position: QueryRange) -> Block<R> {
        unimplemented!()
    }

    pub fn get_block(&self, _hash: &Hash) -> Block<R> {
        unimplemented!()
    }
}
