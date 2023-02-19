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