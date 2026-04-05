use core::num;
use nalgebra::RealField;
use num_traits::Float;
use sha2::{Digest, Sha256};
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
    hash_node: Option<[u8; 32]>,
    left_node: Option<Box<TreeNode>>,
    right_node: Option<Box<TreeNode>>,
}

fn to_hash(unhashed: &str) -> [u8; 32] {
    Sha256::digest(unhashed.as_bytes()).into()
}

impl TreeNode {
    fn new_leaves(transactions: &mut Vec<String>) -> TreeNode {
        let transactions_len: usize = transactions.len() as usize;
        if transactions_len % 2 != 0 {
            let last = transactions[transactions_len - 1].clone();
            transactions.push(last);
        }

        let transactions_hashed: Vec<[u8; 32]> =
            transactions.iter().map(|tx| to_hash(tx.as_str())).collect();

        let mut nodes: Vec<TreeNode>;

        let leaves: Vec<TreeNode> = transactions_hashed
            .into_iter()
            .map(|hash| TreeNode {
                hash_node: Some(hash),
                left_node: None,
                right_node: None,
            })
            .collect();
        let mut nodes: Vec<TreeNode>;
        for tx in (0..transactions_hashed.len()).step_by(2) {
            let mut hasher = Sha256::new();
            hasher.update(transactions_hashed[tx]);
            hasher.update(transactions_hashed[tx + 1]);

            let new_node_hash: Option<[u8; 32]> = Some(hasher.finalize().into());

            // Create new node
            let mut new_node = TreeNode {
                hash_node: new_node_hash,
                left_node: None,
                right_node: None,
            };

            nodes.push(new_node);
        }

        let mut root: TreeNode = TreeNode {
            hash_node: None,
            left_node: None,
            right_node: None,
        };
    }
}

struct BlockChain {
    block_chain: Vec<Block>,
}
