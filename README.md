# BLOCKCHAIN   
This is a lightweight, general purpose blockchain crate built so that you may implement under your project.

This projects makes use of trait objects which implies that in order to track an item as a block entity, you need to make that item implement the Record trait.

The record trait contains functions like hash and verify for obtaining the sha256 hash of the record object, verify for verifying if the given hash matches the current object's hash. There are a lot more default functions under the Record trait.

There crate provides a macro for generating block instances from the trait objects.
The crate also provides a default transaction struct that you can use to test features
Since each individual records implement hash, the blocks themselves are not hashable.


## libraries used
### rust crates
1. bincode = "1.3.3"
2. rusqlite = { version = "0.28.0", features = ["bundled"] }
3. serde = { version="1.0.152", features = ["derive"] }
4. serde_json = "1.0.93"
5. sha2 = "0.10.6"

### database
5. SQLite

## Example 1
```
use blockchain::{blockchain::{BlockChain, Transaction}, block};

fn main() {
    let record1 = Transaction::new("Einstein", "Galileo", "5000");
    let record2 = Transaction::new("Potter", "Ron", "5000");

    let block = block![record1, record2];
    // Creating a new instance of BlockChain only reestablishes connection 
    // with the existing blockchain
    let blockchain = BlockChain::new();
    let stamp = blockchain.append(block);
    println!("{}", stamp);
}
```
Output1
```
$ cargo run
   Compiling blockchain v0.1.0 (D:\workspace\rust\blockchain)
    Finished dev [unoptimized + debuginfo] target(s) in 2.82s
     Running `target\debug\blockchain.exe`
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002
```

Here You may ignore the leading zeros and note 2.
### Example 2
```
use blockchain::blockchain::{Block, BlockChain, TimeStamp, Transaction};

fn main() {
    let time = TimeStamp { value: 2/*The 2 from Output1/ };
    let bc = BlockChain::new();

    let block: Block<Transaction> = bc.get_block(time);
    println!("{:?}", block);
}
```
Output2
```
$ cargo run
   Compiling blockchain v0.1.0 (D:\workspace\rust\blockchain)
    Finished dev [unoptimized + debuginfo] target(s) in 2.12s
     Running `target\debug\blockchain.exe`
Block { records: [Transaction { src: "Einstein", dst: "Galileo", amount: "5000" }, Transaction { src: "Potter", dst: 
"Ron", amount: "5000" }] }
```
