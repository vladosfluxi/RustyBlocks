use std::time::{SystemTime, UNIX_EPOCH};

use crate::block::{Block, BlockBody, BlockHead};
use crate::merkle::TreeNode;
use crate::transaction::{Transaction, TxOutput};

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
}
