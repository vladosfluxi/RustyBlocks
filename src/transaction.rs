use crate::crypto::double_hash;
use secp256k1::{
    hashes::{sha256, Hash},
    rand, Message, Secp256k1,
};

#[derive(Clone)]
pub struct TxInput {
    pub prev_txid: [u8; 32],
    pub prev_vout: u32,
    pub script_signature: Vec<u8>,
    pub sequence: u32,
}

#[derive(Clone)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: Vec<u8>,
}

#[derive(Clone)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub locktime: u32,
}

impl Transaction {
    pub fn calculate_txid(&self) -> [u8; 32] {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.version.to_le_bytes());

        for input in &self.inputs {
            bytes.extend_from_slice(&input.prev_txid);
            bytes.extend_from_slice(&input.prev_vout.to_le_bytes());
            bytes.extend_from_slice(&input.script_signature);
            bytes.extend_from_slice(&input.sequence.to_le_bytes());
        }

        for output in &self.outputs {
            bytes.extend_from_slice(&output.value.to_le_bytes());
            bytes.extend_from_slice(&output.script_pubkey);
        }

        bytes.extend_from_slice(&self.locktime.to_le_bytes());

        double_hash(&bytes)
    }
}
