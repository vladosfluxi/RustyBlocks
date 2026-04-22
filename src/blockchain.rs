use crate::block::{Block, BlockBody, BlockHead};
use crate::merkle::TreeNode;
use crate::transaction::{Transaction, TxOutput};
use chrono::offset;
use num_bigint::BigUint;
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
            hash_prev: [0u8; 32], // no previous block
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

    pub fn add_block(Chain: &mut Self, trx: &Transaction, diff: u64) {
        let target = calculate_target(diff);
    }
}

pub fn calculate_target(diff: u64) -> [u8; 32] {
    let gen_target = BigUint::from_bytes_be(&GENESIS_TARGET);
    let result = gen_target / diff;

    let bytes = result.to_bytes_be();

    let mut target = [0u8; 32];

    let mut offset = 32 - bytes.len();
    target[offset..].copy_from_slice(&bytes);

    target
}
