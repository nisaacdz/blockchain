use blockchain::blockchain::{Block, BlockChain, TimeStamp, Transaction};

fn main() {
    let time = TimeStamp { value: 2/*The 2 from output of example 1*/ };
    let bc = BlockChain::new();

    let block: Block<Transaction> = bc.get_block(time);
    println!("{:?}", block);
}