use crate::block::{Block, BlockBody, BlockHead};
use crate::merkle::TreeNode;
use crate::transaction::{Transaction, TxOutput};
use num_bigint::BigUint;
use std::iter::chain;
use std::time::{SystemTime, UNIX_EPOCH};

// Maximum target — easiest possible mining condition
const GENESIS_TARGET: [u8; 32] = [
    0x00, 0x00, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let coinbase = Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![TxOutput {
                value: 5_000_000_000,
                script_pubkey: vec![],
            }],
            locktime: 0,
        };

        let coinbase_txid = coinbase.calculate_txid();
        let mut txids = vec![coinbase_txid];
        let merkle_root = TreeNode::build(&mut txids).hash_node;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut header = BlockHead {
            index: 0,
            merkle_root_hash: merkle_root,
            hash: [0u8; 32],
            hash_prev: [0u8; 32],
            difficulty: 1,
            timestamp,
            nonce: 0,
        };

        header.mining(GENESIS_TARGET);

        let genesis = Block {
            header,
            body: BlockBody {
                transactions: vec![coinbase],
            },
        };

        BlockChain {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(Chain: &mut Self, trx: &Vec<Transaction>, diff: u64) {
        let target = calculate_target(diff);

        let index: u64 = (Chain.blocks.last().unwrap().header.index + 1) as u64;

        let txids: Vec<[u8; 32]> = trx.iter().map(|tx| tx.calculate_txid()).collect();
        let coinbase = Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![TxOutput {
                value: block_reward(index),
                script_pubkey: vec![],
            }],
            locktime: 0,
        };

        let coinbase_txid = coinbase.calculate_txid();
        let mut all_txids = vec![coinbase_txid];
        all_txids.extend(txids);
        let merkle_root = TreeNode::build(&mut all_txids).hash_node;
        let hash_prev = Chain.blocks.last().unwrap().header.hash;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut blockhead = BlockHead {
            index: index as u32,
            merkle_root_hash: merkle_root,
            hash_prev: hash_prev,
            hash: [0u8; 32],
            timestamp: timestamp,
            difficulty: diff as u8,
            nonce: 0,
        };

        blockhead.mining(target);

        let blockbody = BlockBody {
            transactions: std::iter::once(coinbase)
                .chain(trx.iter().cloned())
                .collect(),
        };

        let block = Block {
            body: blockbody,
            header: blockhead,
        };

        Chain.blocks.push(block);
    }

    pub fn validate_chain(&self) -> bool {}
    pub fn validate_hash_ser(&self) -> bool {}
    pub fn validate_hashes(&self) -> bool {
        for i in self.blocks.windows(2) {
            let current = &i[0];
            let next = &i[1];

            let currect_hash = current.header.hash;
            let next_hash = next.header.hash_prev;

            if currect_hash != next_hash {
                return false;
            }
        }
        true
    }
}
pub fn calculate_target(diff: u64) -> [u8; 32] {
    let gen_target = BigUint::from_bytes_be(&GENESIS_TARGET);
    let result = gen_target / diff;

    let bytes = result.to_bytes_be();

    let mut target = [0u8; 32];

    let offset = 32 - bytes.len();
    target[offset..].copy_from_slice(&bytes);

    target
}

pub fn block_reward(index: u64) -> u64 {
    5_000_000_000 >> (index / 210_000)
}
