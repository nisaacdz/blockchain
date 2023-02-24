use blockchain::{
    block,
    blockchain::{Block, BlockChain, Record, SignedRecord},
    gen,
    utils::{SqliteDB2, Transaction},
};

fn main() {
    // (public key, private key)
    let (public_key, private_key) = gen::generate_key_pair();

    let trans1: Transaction = Transaction::new("A", "B", "2");
    let trans2: Transaction = Transaction::new("B", "A", "5");

    let signed_trans1: SignedRecord<Transaction> = trans1.sign(&private_key, &public_key).unwrap();
    let signed_trans2: SignedRecord<Transaction> = trans2.sign(&private_key, &public_key).unwrap();

    let block: Block<Transaction> = block![signed_trans1, signed_trans2];

    let mut blockchain: BlockChain<SqliteDB2> = BlockChain::open(SqliteDB2::new(r"db\data.db"));

    match blockchain.push(&block) {
        Ok(feedback) => println!("Success! {:?}", feedback),
        Err(err) => println!("Failure! {:?}", err),
    }
}
