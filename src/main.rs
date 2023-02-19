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