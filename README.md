# BLOCKCHAIN   
This is a lightweight, general purpose blockchain crate built so that you may implement under your project.

This projects makes use of trait objects which implies that in order to track an item as a block entity, you need to make that item implement the Record trait.

The record trait contains functions like hash and verify for obtaining the sha256 hash of the record object, verify for verifying if the given hash matches the current object's hash. There are a lot more default functions under the Record trait.

There crate provides a macro for generating block instances from the trait objects.
The crate also provides a default transaction struct that you can use to test features.
Since each individual records implement hash, the blocks themselves are not hashable.

You have the ability to implement the database of your choice


## libraries used
### rust crates
bincode = "1.3.3"
ring = "0.16.20"
serde = { version="1.0.152", features = ["derive"] }
serde_json = "1.0.93"
sha2 = "0.10.6"

### database
rusqlite = { version = "0.28.0", features = ["bundled"] }

## Example 1
```
use blockchain::{utils::Transaction, block, blockchain::{Record, BlockChain}, gen, io::Database};

    fn main() {
        let mykeys = gen::generate_key_pair();
        let trans = Transaction::new("A", "B", "2");

        let signed_record = trans.sign(&mykeys.1, &mykeys.0).unwrap();
        let block = block![signed_record];

        let database = Database::open(None);
        let chain = BlockChain::open(database);

        let res = chain.push(block);

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
