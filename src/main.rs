mod block;
mod blockchain;
mod crypto;
mod merkle;
mod transaction;

fn main() {
    test_genesis_block();
}

// ─── TEMPORARY TEST — remove this function when done ─────────────────────────

fn test_genesis_block() {
    use blockchain::BlockChain;

    println!("Mining genesis block...");

    let chain = BlockChain::new();
    let genesis = &chain.blocks[0];

    println!("Genesis block mined!");
    println!("  index:     {}", genesis.header.index);
    println!("  nonce:     {}", genesis.header.nonce);
    println!("  timestamp: {}", genesis.header.timestamp);
    println!("  hash:      {}", hex(&genesis.header.hash));
    println!("  hash_prev: {}", hex(&genesis.header.hash_prev));
    println!("  merkle:    {}", hex(&genesis.header.merkle_root_hash));
    println!("  txs:       {}", genesis.body.transactions.len());

    // Verify the hash is actually below the target
    let target: [u8; 32] = [
        0x00, 0x00, 0x0F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ];
    assert!(genesis.header.hash <= target, "Hash does not meet target!");
    println!("  target check: PASSED");
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
