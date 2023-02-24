use std::{fmt::Debug, ops::Deref};

use serde::Serialize;
use sha2::{
    digest::{
        generic_array::GenericArray,
        typenum::{UInt, UTerm, B0, B1},
    },
    Digest, Sha256,
};

use crate::errs::CustomErrs;

type Hxsh = GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

#[derive(PartialEq, Eq)]
pub struct Hash {
    data: Hxsh,
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

pub fn encrypt<T: Sized + Serialize>(data: &T) -> Hash {
    let bytes = bincode::serialize(data).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let data: Hxsh = hasher.finalize();

    Hash { data }
}

pub fn validate<T: Sized + Serialize>(obj: &T, hash: Hash) -> bool {
    hash.data == encrypt(obj).data
}

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature, Signer, Verifier};

///
///  Generates ed25519 key pairs in the form (public_key, private_key)
///  # Example
/// ```
///
/// let (public_key, private_key): (Vec<u8>, Vec<u8>) = gen::generate_key_pair();
/// ```
///

pub fn generate_key_pair() -> (Vec<u8>, Vec<u8>) {
    // Generate a new key pair
    let mut c = rand::rngs::OsRng;
    let keypair = Keypair::generate(&mut c);

    // Serialize the private and public keys as byte vectors
    let private_key = keypair.secret.to_bytes().to_vec();
    let public_key = keypair.public.to_bytes().to_vec();

    // Returns the public and private keys as byte vectors
    (public_key, private_key)
}

pub fn sign(msg: &[u8], key: &[u8]) -> Result<Vec<u8>, CustomErrs> {
    // Parse the private key
    match SecretKey::from_bytes(key) {
        Ok(secret) => {
            // Create a signature for the message
            let keypair = Keypair {
                public: PublicKey::from(&secret),
                secret,
            };
            let signature = keypair.sign(msg);
            Ok(signature.as_ref().to_vec())
        }
        Err(_) => Err(CustomErrs::InvalidPrivateKey),
    }
}

pub fn verify_signature(public_key: &[u8], msg: &[u8], signature: &[u8]) -> Result<(), CustomErrs> {
    let dalek = Signature::from_bytes(signature).map_err(|_| CustomErrs::InvalidSignature)?;
    match PublicKey::from_bytes(public_key) {
        Ok(key) => match key.verify(msg, &dalek) {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomErrs::VerificationDoesNotMatch),
        },
        Err(_) => Err(CustomErrs::InvalidPublicKey),
    }
}
