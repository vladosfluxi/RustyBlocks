use crate::crypto::double_hash;
use crate::transaction::Transaction;
use core::time;

pub struct Block {
    pub header: BlockHead,
    pub body: BlockBody,
}

pub struct BlockHead {
    pub index: u32,
    pub merkle_root_hash: [u8; 32],
    pub hash: [u8; 32],
    pub hash_prev: [u8; 32],
    pub difficulty: u8,
    pub timestamp: u64,
    pub nonce: u64,
}

pub struct BlockBody {
    pub transactions: Vec<Transaction>,
}

impl BlockHead {
    pub fn serialize(&self) -> [u8; 32] {
        let mut unhashed = Vec::new();

        unhashed.extend_from_slice(&self.index.to_le_bytes());
        unhashed.extend_from_slice(&self.merkle_root_hash);
        unhashed.extend_from_slice(&self.hash_prev);
        unhashed.extend_from_slice(&self.difficulty.to_le_bytes());
        unhashed.extend_from_slice(&self.timestamp.to_le_bytes());
        unhashed.extend_from_slice(&self.nonce.to_le_bytes());

        double_hash(&unhashed)
    }

    pub fn mining(&mut self, genesis_target: [u8; 32]) {
        loop {
            let hash = self.serialize();
            if hash <= genesis_target {
                self.hash = hash;
                return;
            }
            self.nonce += 1;
        }
    }
}
