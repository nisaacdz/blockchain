use std::{fmt::Debug, marker::PhantomData};

use gen::Signature;
use serde::{Deserialize, Serialize};

use crate::{
    errs::Errs,
    gen,
    gen::Hash,
    io::{Database, QueryRange},
};

#[derive(Debug, Serialize)]
pub struct SignedRecord<T: Record> {
    pub public_key: Vec<u8>,
    pub signature: Signature,
    pub record: T,
}

impl<T: Record> SignedRecord<T> {
    fn verify(&self) -> Result<(), Errs> {
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

#[derive(Debug, Serialize)]
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

    pub fn verify(self) -> Result<VerifiedBlock<R>, Errs> {
        if self.signed_records.iter().all(|r| r.is_valid()) {
            let hash = gen::encrypt(&self);
            Ok(VerifiedBlock { hash, block: self })
        } else {
            Err(Errs::InvalidBlock)
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

pub struct FeedBack {
    pub block_position: QueryRange,
    pub hash: Hash,
}

impl Debug for FeedBack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeedBack")
            .field("block_position", &self.block_position)
            .field("hash", &self.hash)
            .finish()
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
        //keys in the block chain are the BlockPositions, values is dbg!(block)
        Self {
            database,
            phantom_r: PhantomData,
        }
    }

    fn append(&self, block: Block<R>) -> Result<QueryRange, Errs> {
        self.database.insert_block(block)
    }

    fn record(&self, hash: &Hash, block_position: QueryRange) -> Result<(), Errs> {
        self.database.insert_hash(hash, block_position)
    }

    pub fn push(&self, block: Block<R>) -> Result<FeedBack, Errs> {
        let VerifiedBlock { hash, block } = block.verify()?;
        let block_position = self.append(block)?;
        self.record(&hash, block_position)?;
        Ok(FeedBack {
            hash,
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
