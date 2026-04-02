use std::time::{SystemTime, UNIX_EPOCH};

struct Block {
    header: BlockHead,
    body: BlockBody,
}

struct BlockHead {
    index: u8,
    merkle_root_hash: u32,
    hash: u32,
    hash_prev: u32,
    difficulty: u8,
    timestamp: u64,
}

struct BlockBody {
    transactions_counter: u64,
    transactions: TreeNode,
}

struct TreeNode {
    hash_node: u64,
    left_node: Box<Option<TreeNode>>,
    right_node: Box<Option<TreeNode>>,
}

struct BlockChain {
    block_chain: Vec<Block>,
}
