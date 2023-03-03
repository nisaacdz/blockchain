use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{Arc, Mutex},
};

use crate::{
    chain::{Block, BlockChain, FeedBack, Record, SignedRecord},
    errs::CustomErrs,
    io::Database2,
    utils::Entity,
};

#[derive(Clone, Debug)]
pub struct NodeId {
    pub id: u128,
    pub address: String,
}

pub struct Node<D: Database2, R: Record> {
    /// An instance of the blockchain held by this node
    pub chain: Arc<Mutex<BlockChain<D>>>,

    /// Contains a unique identifier for this Node
    /// and it's associated Ip Address
    pub id: NodeId,

    /// Connected peers
    pub peers: HashSet<Box<dyn Entity<R>>>,

    /// A set of unconfirmed records held by this Node
    pub mem_pool: Arc<Mutex<Vec<SignedRecord<R>>>>,

    /// A map of confirmed and published records cast and signed by each user
    /// Only records between members of this Node are kept within this node
    pub transactions: Arc<Mutex<HashMap<u32, HashSet<R>>>>,

    /// A copy of the blockchain containing records that
    /// are relevant to peers in this Node
    ///
    pub local_chain: BlockChain<D>,

    /// Network of nodes connected to this node
    ///
    /// A network can be for diverse purposes
    pub network: Arc<Mutex<Vec<NodeId>>>,
}

#[allow(unused)]
impl<D: Database2, R: Record> Node<D, R> {
    pub fn new() -> Self {
        Self {
            chain: todo!(),
            id: todo!(),
            peers: todo!(),
            mem_pool: todo!(),
            transactions: todo!(),
            local_chain: todo!(),
            network: todo!(),
        }
    }

    pub fn connect_peer(&mut self, peer: Box<dyn Entity<R>>)
    where
        Box<dyn Entity<R>>: Hash + Eq,
    {
        self.peers.insert(peer);
    }

    /// Makes all peers in the Node aware of the published block
    pub fn broadcast_block(&mut self, feed_back: FeedBack<R>) {
        self.peers
            .iter()
            .for_each(|peer| peer.receive_broadcast(&feed_back, self.id.clone()));
        self.push_local(feed_back.get_block()).unwrap();
    }

    pub fn publish_block(&self, block: Block<R>) -> Result<FeedBack<R>, CustomErrs> {
        self.chain.lock().unwrap().push(&block)
    }

    pub fn synchronize(&self) -> bool {
        todo!()
    }

    /// Puts the block in the local database
    /// Local Database contains records that are relevant to members of this Node
    pub fn push_local(&mut self, block: &Block<R>) -> Result<FeedBack<R>, CustomErrs> {
        self.local_chain.push(block)
    }
}

///
/// Struct for managing communications between entities and Nodes
///
///
///
///
///

pub struct Mng<D: Database2> {
    pub db: D,
}
