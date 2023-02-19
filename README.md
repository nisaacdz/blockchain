# BLOCKCHAIN   

# Rust Blockchain Implementation

This is a simple, lightweight blockchain implementation written in Rust that abstracts all the complexities associated with manipulating the blockchain so that you will only focus on your program logic.

## Features

- Uses the `rusqlite` crate for data storage and retrieval
- Supports transaction signing and verification using ed25519
- Uses the `bincode`, `serde` and `serde_json` crate for serialization and deserialization of data
- Uses the `ed25519-dalek` for generating keys, signing records, verifying those records

## Installation

1. Install Rust and Cargo
3. Run the following command to the crate to your project: `cargo add blockchain_dz`


## Usage

The blockchain implementation can be used to create and manage a decentralized ledger of transactions. Most functionalities are enabled through generics and traits so that you can wrap around your custom structs.

Transactions can be signed and verified using ed25519.

`Record` types can be encrypted with sha256 or other algorithms

`Record` types are signable into `SignedRecords` so block are not signable. A block will only be allowed on top of the blockchain if and only if all the SignedRecords contained within it are valid.

The `blockchain` module provides the core functionality for creating and managing the blockchain, while the `io` module provides the interface for storing and retrieving blocks.

The `gen` module wraps the hashing and key generation

The `utils` module contains a simple `Transaction` struct mainly for the purpose of illustrating the core functionalities of this crate.

## Contributing

Contributions to this project are welcome. If you find a bug or want to suggest an improvement, please create an issue or submit a pull request.

## License

This project is licensed under the MIT License.



## Example
```
use blockchain::{utils::Transaction, block, blockchain::{Record, BlockChain, SignedRecord, Block}, gen, io::{Database, TimeStamp}, errs::Errs};

    fn main() {
        let mykeys: (Vec<u8>, Vec<u8>) = gen::generate_key_pair();
        let trans: Transaction = Transaction::new("A", "B", "2");

        let signed_record: SignedRecord<Transaction> = trans.sign(&mykeys.1, &mykeys.0).unwrap();
        let block: Block<Transaction> = block![signed_record];

        // You can pass your database connection to this wrapper
        let database: Database = Database::open(None);

        let chain: BlockChain = BlockChain::open(database);

        let res: Result<TimeStamp, Errs> = chain.push(block);

        println!("{:?}", res);

        // DataBase structure
        /*
        TimeStamp -> Primary Key
        Record -> encrypted or unencrypted message
        Identity -> Public Key
        Signature -> Digital Signature
        // requires private key to decrypt the Record if the record is an encrypted one
        */
    }
```
