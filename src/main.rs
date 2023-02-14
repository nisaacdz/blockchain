use blockchain::block;
use blockchain::blockchain::{Record, Transaction, BlockChain, TimeStamp, Block};
use rusqlite::Connection;

fn main() {
    let time = TimeStamp {value: 1};
    let bc = BlockChain::new();

    let block: Block<Transaction> = bc.get_block(time);
    println!("{:?}", block);
}
