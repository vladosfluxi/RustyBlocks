use crate::blockchain::BlockChain;
mod block;
mod blockchain;
mod crypto;
mod merkle;
mod transaction;

fn main() {
    test_chain_growth();
}

// ─── TEMPORARY TEST — remove this function when done ─────────────────────────

fn test_chain_growth() {
    println!("Building chain with 3 blocks...\n");

    let mut chain = BlockChain::new();
    BlockChain::add_block(&mut chain, &vec![], 1);
    BlockChain::add_block(&mut chain, &vec![], 1);

    for (i, block) in chain.blocks.iter().enumerate() {
        println!("--- Block {} ---", i);
        println!("  index:     {}", block.header.index);
        println!("  nonce:     {}", block.header.nonce);
        println!("  difficulty:{}", block.header.difficulty);
        println!("  hash:      {}", hex(&block.header.hash));
        println!("  hash_prev: {}", hex(&block.header.hash_prev));
        println!("  merkle:    {}", hex(&block.header.merkle_root_hash));
        println!("  txs:       {}", block.body.transactions.len());
        println!();
    }

    // ─── Checks ───
    assert_eq!(chain.blocks.len(), 3, "Chain should have 3 blocks");

    for i in 1..chain.blocks.len() {
        assert_eq!(
            chain.blocks[i].header.hash_prev,
            chain.blocks[i - 1].header.hash,
            "Block {} hash_prev does not match block {} hash",
            i,
            i - 1
        );
        assert_eq!(
            chain.blocks[i].header.index,
            chain.blocks[i - 1].header.index + 1,
            "Block {} index is not sequential",
            i
        );
    }

    // Each block must have at least 1 tx (the coinbase)
    for (i, block) in chain.blocks.iter().enumerate() {
        assert!(
            !block.body.transactions.is_empty(),
            "Block {} has no transactions",
            i
        );
    }

    println!("ALL CHECKS PASSED — chain is linked, indexed, and has coinbases");
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
