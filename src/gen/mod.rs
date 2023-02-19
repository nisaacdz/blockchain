use std::{ops::Deref, fmt::{Debug, Display}};

use serde::Serialize;
use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Sha256,
};

use crate::errs::Errs;

type Hxsh = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

#[derive(PartialEq, Eq)]
pub struct Hash {
    data: Hxsh,
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl Deref for Hash {
    type Target = Hxsh;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub fn encrypt<T: Sized + Serialize>(data: T) -> Hash {
    let bytes = bincode::serialize(&data).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let data: Hxsh = hasher.finalize();

    Hash { data }
}

pub fn validate<T: Sized + Serialize>(obj: T, hash: Hash) -> bool {
    hash.data == encrypt(obj).data
}

use ring::{rand::SystemRandom, signature::Signature, agreement::UnparsedPublicKey };
use ring::signature::{self, Ed25519KeyPair};

pub fn generate_key_pair() -> Ed25519KeyPair {
    let rng = SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap().as_ref().to_vec();
    Ed25519KeyPair::from_pkcs8(&pkcs8).unwrap()
}


pub fn sign(msg: &[u8], key: &[u8]) -> Result<Signature, Errs> {
    match Ed25519KeyPair::from_seed_unchecked(key) {
        Ok(key) => Ok(key.sign(msg)),
        Err(_) => Err(Errs::InvalidKey),
    }
}

pub fn verify_signature(public_key_bytes: &[u8], msg: &[u8], signature: &[u8]) -> Result<(), Errs> { 
    let key =  signature::UnparsedPublicKey::new(&signature::ED25519, public_key_bytes);
    match key.verify(msg, signature) {
        Ok(_) => Ok(()),
        Err(_) => Err(Errs::Default),
    }
}
