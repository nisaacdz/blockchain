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
