use crate::transaction::Transaction;

pub struct Block {
    pub header: BlockHead,
    pub body:   BlockBody,
}

pub struct BlockHead {
    pub index:            u32,
    pub merkle_root_hash: [u8; 32],
    pub hash:             [u8; 32],
    pub hash_prev:        [u8; 32],
    pub difficulty:       u8,
    pub timestamp:        u64,
    pub nonce:            u64,
}

pub struct BlockBody {
    pub transactions: Vec<Transaction>,
}
