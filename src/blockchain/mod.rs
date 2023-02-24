use serde::{Deserialize, Serialize};

use crate::{
    errs::CustomErrs,
    gen,
    gen::Hash,
    io::{Database2, DatabaseInsertable, QueryRange},
};

///
///
///
///
///
///
///

static RECORDS_COLUMNS: [&'static str; 3] = ["Record", "Identity", "Signature"];
static BLOCKS_COLUMNS: [&'static str; 2] = ["Hash", "Range"];
static RECORDS: &'static str = "RECORDCHAIN";
static BLOCKS: &'static str = "BLOCKCHAIN";

#[derive(Debug, Serialize, Clone)]
pub struct SignedRecord<T: Record> {
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
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

    pub fn to_vec(&self) -> Vec<String> {
        vec![
            serde_json::to_string(&self.record).unwrap(),
            serde_json::to_string(&self.public_key).unwrap(),
            serde_json::to_string(&self.signature).unwrap(),
        ]
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    pub fn get_record(&self) -> &T {
        &self.record
    }

    pub fn get_signer(&self) -> &Vec<u8> {
        &self.public_key
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
///
///
///
///
///

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
///
///
///
///
///
///
///
///
///
///
///
///
#[derive(Debug, Serialize, Clone)]
pub struct Block<R: Record> {
    pub signed_records: Vec<SignedRecord<R>>,
}

impl<R: Record> DatabaseInsertable for &Block<R> {
    fn get_name() -> &'static str {
        &RECORDS
    }

    fn columns() -> &'static [&'static str] {
        &RECORDS_COLUMNS
    }

    fn len(&self) -> i64 {
        self.size()
    }
}

pub struct IterBlockString<'a, R: Record> {
    pos: usize,
    records: &'a Vec<SignedRecord<R>>,
}

impl<'a, R: Record> Iterator for IterBlockString<'a, R> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.records.len() {
            self.pos += 1;
            Some(self.records[self.pos - 1].to_vec())
        } else {
            None
        }
    }
}

impl<'a, R: Record> IntoIterator for &'a Block<R> {
    type Item = Vec<String>;

    type IntoIter = IterBlockString<'a, R>;

    fn into_iter(self) -> Self::IntoIter {
        IterBlockString {
            pos: 0,
            records: self.get_signed_records(),
        }
    }
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

    pub fn get_signed_records(&self) -> &Vec<SignedRecord<R>> {
        &self.signed_records
    }

    pub fn verify(&self) -> Result<VerifiedBlock<R>, CustomErrs> {
        if self.signed_records.iter().all(|r| r.is_valid()) {
            let hash = gen::encrypt(&self);
            Ok(VerifiedBlock {
                block: self.clone(),
                hash: hash.to_vec(),
            })
        } else {
            Err(CustomErrs::InvalidBlock)
        }
    }
}

pub struct VerifiedBlock<R: Record> {
    pub hash: Vec<u8>,
    pub block: Block<R>,
}

impl<R: Record> VerifiedBlock<R> {
    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn get_block(&self) -> &Block<R> {
        &self.block
    }
}

#[derive(Debug)]
pub struct FeedBack<R: Record> {
    pub block_position: QueryRange,
    pub hash: Vec<u8>,
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

pub struct PublishedBlock {
    hash: Vec<u8>,
    block_position: QueryRange,
}

impl PublishedBlock {
    pub fn to_vec(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.hash),
            serde_json::to_string(&self.block_position).unwrap(),
        ]
    }
}

pub struct ItemsIter<'a> {
    picked: bool,
    items: &'a PublishedBlock,
}

impl<'a> Iterator for ItemsIter<'a> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.picked {
            false => {
                self.picked = false;
                Some(self.items.to_vec())
            }
            true => None,
        }
    }
}

impl<'a> IntoIterator for &'a PublishedBlock {
    type Item = Vec<String>;

    type IntoIter = ItemsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ItemsIter {
            picked: false,
            items: &self,
        }
    }
}

impl DatabaseInsertable for &PublishedBlock {
    fn get_name() -> &'static str {
        &BLOCKS
    }

    fn columns() -> &'static [&'static str] {
        &BLOCKS_COLUMNS
    }

    fn len(&self) -> i64 {
        1
    }
}

pub struct BlockChain<D: Database2> {
    database: D,
}

impl<D: Database2> BlockChain<D> {
    pub fn open(database: D) -> Self {
        Self { database }
    }

    fn append<R: Record>(&mut self, block: &Block<R>) -> Result<QueryRange, CustomErrs> {
        self.database.insert(&block)
    }

    fn record(&mut self, published_block: &PublishedBlock) -> Result<QueryRange, CustomErrs> {
        self.database.insert(&published_block)
    }

    pub fn push<R: Record>(&mut self, block: &Block<R>) -> Result<FeedBack<R>, CustomErrs> {
        if block.size() == 0 {
            return Err(CustomErrs::EmptyBlocksNotAllowed);
        }

        let VerifiedBlock { hash, block } = block.verify()?;
        let block_position = self.append(&block)?;
        let published_block = PublishedBlock {
            hash: hash.clone(),
            block_position,
        };
        self.record(&published_block)?;
        Ok(FeedBack {
            hash,
            block: block.clone(),
            block_position,
        })
    }

    pub fn get_records<R: Record>(&self, _block_position: QueryRange) -> Block<R> {
        unimplemented!()
    }

    pub fn get_block<R: Record>(&self, _hash: &Hash) -> Block<R> {
        unimplemented!()
    }
}
