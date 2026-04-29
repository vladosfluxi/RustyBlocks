use crate::blockchain::BlockChain;
use crate::transaction::{Transaction, TxInput, TxOutput};
mod block;
mod blockchain;
mod crypto;
mod merkle;
mod transaction;

fn main() {
    let mut block_chain = BlockChain::new();
    BlockChain::add_block(&mut block_chain, &example_transactions(), 1);
    println!("{}", block_chain.validate_chain());
    block_chain.blocks[1].body.transactions[0].outputs[0].value = 999_999_999;
    println!("after tamper: {}", block_chain.validate_chain());
}
fn example_transactions() -> Vec<Transaction> {
    vec![
        Transaction {
            version: 1,
            inputs: vec![TxInput {
                prev_txid: [0xAA; 32],
                prev_vout: 0,
                script_signature: vec![],
                sequence: 0xFFFFFFFF,
            }],
            outputs: vec![
                TxOutput {
                    value: 50_000_000,
                    script_pubkey: vec![1, 2, 3],
                }, // Bob
                TxOutput {
                    value: 50_000_000,
                    script_pubkey: vec![4, 5, 6],
                }, // Alice change
            ],
            locktime: 0,
        },
        // Transaction 2 — Combines 2 inputs into 1 output (consolidation)
        Transaction {
            version: 1,
            inputs: vec![
                TxInput {
                    prev_txid: [0xBB; 32],
                    prev_vout: 0,
                    script_signature: vec![],
                    sequence: 0xFFFFFFFF,
                },
                TxInput {
                    prev_txid: [0xCC; 32],
                    prev_vout: 1,
                    script_signature: vec![],
                    sequence: 0xFFFFFFFF,
                },
            ],
            outputs: vec![TxOutput {
                value: 200_000_000,
                script_pubkey: vec![7, 8, 9],
            }],
            locktime: 0,
        },
        // Transaction 3 — One input split into 3 outputs (payment + 2 change-like)
        Transaction {
            version: 1,
            inputs: vec![TxInput {
                prev_txid: [0xDD; 32],
                prev_vout: 2,
                script_signature: vec![],
                sequence: 0xFFFFFFFF,
            }],
            outputs: vec![
                TxOutput {
                    value: 30_000_000,
                    script_pubkey: vec![10],
                },
                TxOutput {
                    value: 20_000_000,
                    script_pubkey: vec![11],
                },
                TxOutput {
                    value: 10_000_000,
                    script_pubkey: vec![12],
                },
            ],
            locktime: 0,
        },
    ]
}
