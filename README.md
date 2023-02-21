# BLOCKCHAIN   

# Rust Blockchain Implementation

This is a simple, lightweight blockchain implementation written in Rust that abstracts all the complexities associated with implementing and manipulating blockchain so that you only have to focus on your program logic.

## Features

- Uses the `rusqlite` crate for data storage and retrieval
- Supports transaction signing and verification using ed25519
- Uses the `bincode`, `serde` and `serde_json` crate for serialization and deserialization of data
- Uses the `ed25519-dalek` for generating keys, signing records, and verifying those records

## Installation

1. ________


## Usage

The blockchain implementation can be used to create and manage a decentralized ledger of transactions. Most functionalities are enabled through generics and traits. 


To get started, check the documentations to see which traits to implement and which types to use in the generic structs


Transactions can be signed and verified using ed25519.

`Record` types can be encrypted with sha256 or other algorithms


`Record` types are signable into `SignedRecords`.

`Block` types consist of one or more `SignedRecords`.

`Block` types can be verified and placed on the blockchain.

The `blockchain` module provides the core functionality for creating and managing the blockchain.

The `io` module provides the interface for storing and retrieving blocks.

The `gen` module wraps the hashing and key generation

The `utils` module contains a simple implementations that you will find handy.

The `errs` module contains custom `Err` types used in the crate.


## Contributing

Contributions to this project are welcome. If you find a bug or want to suggest an improvement, please create an issue or submit a pull request.


## License

This project is licensed under the MIT License.



## Example

```

use blockchain::{
    block,
    blockchain::{Block, BlockChain, Record, SignedRecord},
    gen,
    utils::{SqliteDB, Transaction},
};

fn main() {
    // (public key, private key)
    let (public_key, private_key) = gen::generate_key_pair();

    let trans1: Transaction = Transaction::new("A", "B", "2");
    let trans2: Transaction = Transaction::new("B", "A", "5");

    let signed_trans1: SignedRecord<Transaction> = trans1.sign(&private_key, &public_key).unwrap();
    let signed_trans2: SignedRecord<Transaction> = trans2.sign(&private_key, &public_key).unwrap();

    let block: Block<Transaction> = block![signed_trans1, signed_trans2];

    // You can pass your database connection to this wrapper
    let database: SqliteDB = SqliteDB::open(r"db\database.db").unwrap();

    let blockchain: BlockChain<SqliteDB, Transaction> = BlockChain::open(database);

    match blockchain.push(block) {
        Ok(feedback) => println!("Success! {:?}", feedback),
        Err(err) => println!("Failure! {:?}", err),
    }

    // DataBase structure

    //Table 1 name = records
    /*
    Position -> Primary Key number
    Record -> encrypted or unencrypted message text
    Identity -> Public Key text
    Signature -> Digital Signature text
    // requires private key to decrypt the Record if the record is an encrypted one

    //Table 2 name = hash
    Hash -> Primary Key text
    BlockPosition -> text
    */
}



```


Output

```

Success! FeedBack { block_position: QueryRange { begin: 4, end: 5 }, hash: [136, 62, 191, 230, 174, 233, 24, 32, 130, 97, 247, 86, 9, 138, 161, 243, 127, 183, 210, 135, 173, 174, 184, 84, 61, 197, 122, 13, 25, 33, 154, 72] }


```
