use blockchain::{utils::Transaction, block, blockchain::Record, gen};

fn main() {
    let mykeys = gen::generate_key_pair();
    let trans = Transaction::new("A", "B", "2");
    let single_block = block![trans.sign(&mykeys.1, &mykeys.0).unwrap()];
}