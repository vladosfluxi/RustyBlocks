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
#[derive(Clone)]
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
        let transactions_len: usize = transactions.clone().len() as usize;
        if transactions_len % 2 != 0 {
            let last = transactions[transactions_len - 1].clone();
            transactions.push(last);
        }

        let transactions_hashed: Vec<[u8; 32]> =
            transactions.iter().map(|tx| to_hash(tx.as_str())).collect();

        let leaves: Vec<TreeNode> = transactions_hashed
            .clone()
            .into_iter()
            .map(|hash| TreeNode {
                hash_node: Some(hash),
                left_node: None,
                right_node: None,
            })
            .collect();

        let mut nodes: Vec<TreeNode> = vec![];

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

        // let mut root: TreeNode = TreeNode {
        //     hash_node: None,
        //     left_node: None,
        //     right_node: None,
        // };
        loop {
            if nodes.len() <= 1 {
                break;
            }

            if nodes.len() % 2 != 0 {
                let nodes_last: TreeNode = nodes[nodes.len() - 1].clone();

                nodes.push(nodes_last);
            }

            let mut next_level: Vec<TreeNode> = Vec::new();

            for i in (0..nodes.len()).step_by(2) {
                let mut left_node = nodes[i].clone();
                let mut right_node = nodes[i + 1].clone();

                let mut left_hash = left_node.hash_node.clone();
                let mut right_hash = right_node.hash_node.clone();

                let mut hasher = Sha256::new();
                hasher.update(left_hash.unwrap());
                hasher.update(right_hash.unwrap());

                let parent_hash: [u8; 32] = hasher.finalize().into();

                let parent_node = TreeNode {
                    hash_node: Some(parent_hash),
                    left_node: Some(Box::new(left_node)),
                    right_node: Some(Box::new(right_node)),
                };

                next_level.push(parent_node);
            }
            nodes = next_level;
        }
        nodes.pop().unwrap()
    }
}

struct BlockChain {
    block_chain: Vec<Block>,
}
