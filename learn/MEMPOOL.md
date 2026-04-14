# The Memory Pool (Mempool) — An Ultra Detailed Guide

---

## Abbreviations

| Abbreviation | Stands For |
|---|---|
| mempool | Memory Pool — the waiting room for unconfirmed transactions |
| tx / txs | Transaction / Transactions |
| UTXO | Unspent Transaction Output |
| fee rate | The fee per unit of data, measured in satoshis per virtual byte (sat/vByte) |
| sat / sats | Satoshi / Satoshis — the smallest unit of Bitcoin (1 BTC = 100,000,000 sats) |
| vByte | Virtual Byte — a unit that accounts for transaction size including witness data |
| RBF | Replace By Fee — a mechanism to replace an unconfirmed tx with a higher-fee version |
| CPFP | Child Pays For Parent — a mechanism where a new tx pays a high fee to pull its parent through |
| block template | The candidate block a miner assembles from mempool transactions before mining |
| double spend | Attempting to spend the same UTXO in two different transactions |
| eviction | Removing a transaction from the mempool due to low fee or mempool being full |
| propagation | The process of a transaction spreading across all nodes in the network |

---

## What is the Mempool?

The mempool is a **temporary holding area** for transactions that have been broadcast
to the network but have not yet been included in a block.

Think of it like the waiting room at a doctor's office:

```
Patient walks in (transaction is broadcast)
        ↓
Receptionist checks paperwork (node validates the transaction)
        ↓
Patient sits in the waiting room (transaction enters the mempool)
        ↓
Doctor calls patients in order of priority (miner picks transactions by fee)
        ↓
Patient is seen (transaction is included in a block and confirmed)
```

Every full node on the Bitcoin network maintains its **own mempool**. There is no
single global mempool — each node independently decides which transactions to keep,
based on its own rules and available memory.

---

## There Is No Single Global Mempool

This is a common misconception. The mempool is not one shared database.

Every node has its own:

```
Node A mempool:  [tx1, tx2, tx3, tx4, tx5]
Node B mempool:  [tx1, tx2, tx3,      tx5, tx6]
Node C mempool:  [tx1,      tx3, tx4, tx5, tx6, tx7]
```

Nodes differ because:
- Transactions arrive at different times due to network latency
- Nodes have different memory limits for their mempool
- Some nodes apply stricter filters (minimum fee rate, etc.)
- A transaction may have been evicted from one node but not another

When people say "the mempool is congested" they mean the **average state** across
all nodes — not a single shared object.

---

## The Life of a Transaction — From Creation to Confirmation

### Step 1 — Creation

A user builds a transaction on their wallet:
- Selects UTXOs to spend as inputs
- Specifies outputs (recipient + change address)
- Sets a fee (total input value minus total output value)
- Signs the transaction with their private key

### Step 2 — Broadcast

The wallet sends the raw transaction bytes to one or more nodes it is connected to.

```
User wallet ──▶ Node A ──▶ Node B ──▶ Node C ──▶ ...
                     ──▶ Node D ──▶ Node E ──▶ ...
```

This spreading process is called **propagation**. Within seconds, the transaction
reaches thousands of nodes worldwide.

### Step 3 — Validation by Each Node

Before accepting a transaction into its mempool, every node runs a checklist:

```
✓  Is the transaction format valid? (correct structure, no missing fields)
✓  Are all input UTXOs real and unspent?
✓  Does the total input value >= total output value? (no coins created from nothing)
✓  Is each input's signature valid? (proves the spender owns the coins)
✓  Does the transaction meet the minimum fee rate? (default: 1 sat/vByte)
✓  Is it not already in the mempool or in a confirmed block?
✓  Does it conflict with another mempool transaction? (double spend attempt)
✓  Is the transaction size within limits? (max 400,000 weight units)
✓  Is there enough memory in the mempool to hold it?
```

If any check fails, the transaction is **rejected and dropped**. It does not propagate
further from that node.

If all checks pass, the transaction enters the node's mempool and is forwarded to peers.

### Step 4 — Waiting in the Mempool

The transaction sits in memory, waiting for a miner to pick it up. This can take:
- **Seconds** — if the fee is very high and blocks have space
- **Minutes** — normal conditions with appropriate fee
- **Hours or days** — if the fee is too low or the mempool is congested
- **Forever (eviction)** — if the mempool fills up and the fee is too low

### Step 5 — Picked Up by a Miner

A miner selects the transaction for inclusion in their candidate block.

### Step 6 — Confirmed

The miner finds a valid block containing the transaction and broadcasts it.
Every node that receives the block:
1. Validates the block and all its transactions
2. Adds the block to their chain
3. **Removes every transaction in the block from their mempool**
4. Checks if any remaining mempool transactions are now invalid (e.g. double spends
   that referenced the same UTXOs — they get evicted)

---

## Transaction Fees — The Core of Mempool Priority

Fees are not a fixed cost. They are a **bid in an auction** for block space.

### Why fees exist

A Bitcoin block has a maximum size (~1MB of raw data, ~4MB of weight).
Only a limited number of transactions fit per block.
When more transactions are waiting than can fit, miners must choose.
They choose the ones that pay them the most.

### How fee rate is calculated

```
fee      = total_input_value - total_output_value   (in satoshis)
fee rate = fee / transaction_size                    (in sat/vByte)
```

Example:
```
inputs:  0.001 BTC  = 100,000 sats
outputs: 0.000978 BTC = 97,800 sats
fee:     100,000 - 97,800 = 2,200 sats
tx size: 250 vBytes
fee rate = 2,200 / 250 = 8.8 sat/vByte
```

Miners rank transactions by **fee rate**, not total fee. A tiny transaction paying
500 sats across 100 vBytes (5 sat/vByte) beats a large transaction paying 900 sats
across 400 vBytes (2.25 sat/vByte).

### What determines a "good" fee rate?

There is no fixed answer. It depends entirely on current mempool congestion:

```
Mempool nearly empty:   1-2 sat/vByte is fine, confirms next block
Moderate congestion:    10-20 sat/vByte for next block
High congestion:        50-100+ sat/vByte to get in next block
Extreme congestion:     200+ sat/vByte (e.g. during NFT crazes or halving periods)
```

Wallets query the mempool to estimate the current going rate and suggest a fee.

---

## How the Miner Interacts With the Mempool

This is the critical part — how a miner actually uses the mempool.

### Step 1 — Observing the Mempool

The miner's node has its own mempool, continuously updated as new transactions arrive
and old ones get confirmed or evicted. The miner watches this pool in real time.

### Step 2 — Building a Block Template

When the miner is ready to start mining a new block (after the previous block was
found), they build a **block template** — a candidate block filled with transactions
chosen from the mempool.

The goal: **maximize total fees collected** while staying within the block size limit.

This is essentially a variation of the classic computer science **knapsack problem**:

```
Given a bag with limited capacity (block size limit)
and items of varying size and value (transactions with sizes and fees),
pick the combination of items that maximizes total value (total fees).
```

The standard greedy approach used in Bitcoin:

```
1. Sort all mempool transactions by fee rate (highest first)
2. Start adding transactions from the top of the list
3. Stop when the block is full or the mempool is empty
4. What remains in the mempool stays and waits for the next block
```

### Step 3 — Adding the Coinbase Transaction

The very first transaction in every block is the **coinbase transaction**, created
by the miner themselves:

```
Coinbase transaction:
  input:   nothing (no prev_txid — coins are created from thin air)
  output:  block_reward + total_fees_from_all_included_transactions
           → paid to the miner's own address
```

This is the miner's payment. If the miner includes transactions totalling 0.05 BTC
in fees, plus the 3.125 BTC block reward, their coinbase output is 3.175 BTC.

### Step 4 — Mining the Block

With the block template assembled, the miner starts the mining loop:

```
loop:
  header = serialize(index, prev_hash, merkle_root, timestamp, nBits, nonce)
  hash   = double_hash(header)
  if hash < target:
      broadcast block
      break
  else:
      nonce += 1
```

The transactions in the block do not change during mining — only the nonce changes.
The merkle root commits to all the transactions, so they are fixed once the template
is built.

### Step 5 — Broadcasting the Winning Block

When the miner finds a valid nonce, they broadcast the complete block immediately.
Speed matters — if another miner finds a block first, the race is lost.

### Step 6 — Updating the Mempool After a Block

Once a block is confirmed, every node (including the miner's own node) does this:

```
for each transaction in the new block:
    remove it from the mempool

for each remaining mempool transaction:
    if any of its inputs are now spent (by the confirmed block):
        remove it too  ← this kills double spend attempts
```

The miner immediately starts building the next block template from the updated mempool.

---

## Transaction Ordering Inside a Block

Transactions inside a block are not completely random but follow some rules:

1. **Coinbase must be first** — always, by protocol rule
2. **Parent before child** — if transaction B spends an output from transaction A,
   then A must appear before B in the block (otherwise B references a UTXO that
   doesn't exist yet within the block)
3. **The rest** — ordered by the miner however they like, usually by fee rate

---

## Dependent Transactions — Transaction Chains in the Mempool

Sometimes a transaction in the mempool spends an output from **another unconfirmed
transaction** also in the mempool. These form a chain:

```
tx_A  (confirmed UTXO → output_A)
  ↓
tx_B  (output_A → output_B)    ← unconfirmed, in mempool
  ↓
tx_C  (output_B → output_C)    ← unconfirmed, in mempool, depends on tx_B
```

tx_C cannot be confirmed without tx_B. tx_B cannot be confirmed without tx_A being
on-chain first.

Miners handle this by treating the whole chain as a unit when calculating effective
fee rate. Bitcoin Core limits chains to **25 unconfirmed ancestors** by default.

---

## Replace By Fee (RBF)

Imagine you sent a transaction with too low a fee and it's been stuck for hours.
**RBF** lets you replace it with a new version that pays a higher fee.

Rules for a valid RBF replacement:
- The new transaction must spend at least one of the same inputs as the original
- The new fee rate must be higher than the original (by at least 1 sat/vByte more)
- The new absolute fee must be higher than the original
- The replacement must not introduce more than 100 new unconfirmed ancestors

When nodes see a valid RBF replacement:
- They drop the old transaction from their mempool
- They add the new one
- Miners now see the higher-fee version and prefer it

RBF must be explicitly opted into by setting the `sequence` field of at least one
input to a value less than `0xFFFFFFFE`.

---

## Child Pays For Parent (CPFP)

The opposite problem: someone sent you a transaction with too low a fee and you
want it confirmed faster (because you want to spend the output).

**CPFP** works like this:

```
tx_parent  →  output to you  (stuck in mempool, low fee)
    ↓
tx_child   →  spends that output, pays very high fee
```

Miners see tx_child paying a high fee rate. But tx_child cannot be confirmed without
tx_parent. So the miner considers the **combined fee rate** of both transactions as
a package:

```
combined fee rate = (fee_parent + fee_child) / (size_parent + size_child)
```

If this combined rate is high enough, the miner includes both in the block, even
though tx_parent alone has a low fee. The child "pays for" the parent.

---

## Mempool Eviction — What Happens to Low-Fee Transactions

Each node caps its mempool at a memory limit (default 300 MB in Bitcoin Core).
When the mempool is full and a new transaction arrives:

```
if new_tx.fee_rate > lowest_fee_rate_in_mempool:
    evict the lowest-fee transaction(s) to make room
    add the new transaction

else:
    reject the new transaction entirely
```

Evicted transactions are not invalid — they just got outbid. The sender must either:
- Wait until the mempool clears (during low-traffic periods)
- Rebroadcast with a higher fee (RBF if opted in)
- Do nothing and hope a miner with a lower threshold picks it up eventually

Bitcoin Core also evicts transactions that have been in the mempool for more than
**2 weeks** by default, regardless of fee.

---

## Mempool Congestion — What It Looks Like

When many people are sending transactions simultaneously:

```
Normal state:
  mempool size: ~5,000 transactions
  blocks clear the mempool faster than it fills
  fee rate needed: 1-5 sat/vByte
  confirmation time: next block (~10 min)

Congested state:
  mempool size: 100,000+ transactions
  blocks cannot clear the backlog
  fee rate needed: 50-200+ sat/vByte to get in next block
  low-fee transactions: stuck for hours or days
  eviction threshold rises: very low-fee txs get dropped entirely
```

Famous congestion events:
- **2017 bull run** — fees reached 1,000+ sat/vByte, average tx cost $50+
- **2021 bull run** — fees peaked around 300 sat/vByte
- **2023 Ordinals/inscriptions** — NFT-like data flooded the mempool

---

## Mempool in RustyBlocks — What You Need to Build

For your blockchain, the mempool is simply a collection of validated, unconfirmed
transactions. A minimal implementation:

```rust
struct Mempool {
    transactions: Vec<Transaction>,
}

impl Mempool {
    // Add a validated transaction
    fn add(&mut self, tx: Transaction) {
        self.transactions.push(tx);
    }

    // Remove confirmed transactions after a block is mined
    fn remove_confirmed(&mut self, confirmed_txs: &[Transaction]) {
        self.transactions.retain(|tx| !confirmed_txs.contains(tx));
    }

    // Select transactions for a block template, sorted by fee rate
    fn select_for_block(&self, max_count: usize) -> Vec<Transaction> {
        let mut sorted = self.transactions.clone();
        sorted.sort_by(|a, b| b.fee_rate().cmp(&a.fee_rate()));
        sorted.truncate(max_count);
        sorted
    }
}
```

The miner calls `select_for_block()` to build the block template, mines it, and
upon success calls `remove_confirmed()` to clean the mempool.

---

## Summary

```
Mempool = the waiting room for unconfirmed transactions.
          Every node has its own. There is no global mempool.

Transaction lifecycle:
  broadcast → validate → enter mempool → wait → miner picks it → confirmed → removed

Miner interaction:
  1. Observe mempool
  2. Build block template (sort by fee rate, fill up to block size limit)
  3. Add coinbase transaction (block reward + all fees)
  4. Mine the block (find valid nonce)
  5. Broadcast winning block
  6. Remove confirmed transactions from mempool
  7. Repeat

Fee = your bid for block space.
Higher fee rate = picked sooner.
Too low fee rate = stuck, possibly evicted.

RBF  = replace your stuck tx with a higher-fee version.
CPFP = create a child tx with high fee to pull a stuck parent through.
```
