use rusqlite::Connection;

use crate::blockchain::{Block, Record};

pub struct TimeStamp {
    begin: usize,
    end: usize,
}
impl std::fmt::Debug for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeStamp")
            .field("begin", &self.begin)
            .field("end", &self.end)
            .finish()
    }
}

pub struct Database {
    connection: Option<Connection>,
}

impl Database {
    pub fn open(connection: Option<Connection>) -> Self {
        // r"db\mydatabase.db"
        Self { connection }
    }

    /*
    pub fn verify(
        &self,
        message: &[u8],
        signature: &ed25519::Signature
    ) -> Result<(), SignatureError>
    {
        self.public.verify(message, signature)
    }
    */

    pub fn insert<T: Record>(&self, block: Block<T>) -> TimeStamp {
        unimplemented!()
    }
}
