# Bitcoin Nodes — An Ultra Detailed Guide

---

## Abbreviations

| Abbreviation | Stands For |
|---|---|
| node | Any computer running Bitcoin software and connected to the network |
| P2P | Peer-to-Peer — a network where computers talk directly to each other, no central server |
| full node | A node that downloads and validates every block and transaction since genesis |
| pruned node | A full node that deletes old block data after validation to save disk space |
| mining node | A full node with mining hardware attached that competes to find new blocks |
| SPV | Simplified Payment Verification — a light client that only downloads block headers |
| light node | Same as SPV — a minimal node used in mobile wallets |
| archival node | A full node that keeps every byte of blockchain history forever |
| IBD | Initial Block Download — the process of downloading and validating the entire chain when first joining |
| DNS seed | A domain name that returns IP addresses of known Bitcoin nodes, used for bootstrapping |
| peer | Another node you are directly connected to |
| gossip | The method of spreading information — you tell your peers, they tell theirs, etc. |

---

## What is a Node?

A **node** is any computer that runs Bitcoin software and connects to other computers
running the same software.

That is it. No special hardware. No permission needed. No company owns them.
Your laptop could be one right now if you downloaded Bitcoin Core.

The critical insight is this:

> **The network IS the nodes. There is no server behind it.**

A helpful analogy is **BitTorrent**. When you download a file via BitTorrent, you
are not downloading from a central server — you are downloading pieces from thousands
of other people's computers that happen to be running the same software. If every
single person stopped running BitTorrent tomorrow, the network would cease to exist.
Bitcoin works the same way.

```
CENTRALIZED (like a bank):
  You ──▶ Bank Server ──▶ You
           (single point of failure, owned by someone, can be shut down)

DECENTRALIZED (like Bitcoin):
  You ──▶ Node A ──▶ Node C ──▶ Node E
       ──▶ Node B ──▶ Node D ──▶ Node F
  (no center, no owner, cannot be shut down by attacking one point)
```

When people say Bitcoin is decentralized, this is what they mean: the network
is made of thousands of independently operated computers around the world, none
of which is in charge.

---

## Types of Nodes

Not every node is the same. Nodes have different roles and capabilities:

| Type | Validates Blocks | Stores Full Chain | Mines | Used By |
|---|---|---|---|---|
| Full node | Yes — everything | Yes (~600GB) | No | Enthusiasts, businesses |
| Pruned node | Yes — everything | No (keeps ~10GB) | No | Users with limited disk |
| Mining node | Yes — everything | Yes | Yes | Miners |
| SPV / Light node | No — trusts peers | No (headers only, ~50MB) | No | Mobile wallets |
| Archival node | Yes — everything | Yes + all history | No | Block explorers |

### Full Node

A full node downloads every block that has ever existed, starting from the genesis
block, and **independently validates every single one** — every transaction, every
signature, every rule. It trusts nobody. It checks everything itself.

This is the gold standard of participation. A full node cannot be lied to because
it never takes anyone's word for it — it re-derives the truth from raw data.

Requires ~600GB of disk space and several days to sync from scratch.

### Pruned Node

Same as a full node, but after validating old blocks it **deletes** them to save
disk space. It keeps only the most recent blocks (configurable, minimum ~550MB).

It still validates everything — it just does not keep the old receipts. It is just
as trustworthy as a full node for security purposes.

### Mining Node

A full node with **mining hardware** (ASICs) connected to it. The software part
does everything a full node does, plus:
- Pulls transactions from the mempool
- Builds block templates
- Feeds the template to the ASICs
- Receives winning nonces from the ASICs and broadcasts the finished block

The mining hardware does only the hashing — all the logic is in the node software.

### SPV / Light Node

A light node only downloads **block headers** (80 bytes each), not full blocks.
Block headers are enough to verify that a block exists and links to the chain,
but not enough to verify transactions independently.

Light nodes **ask full nodes** whether a transaction was included in a block.
They trust the answer. This is a security trade-off for the sake of speed and
storage — mobile wallets cannot hold 600GB.

### Archival Node

A full node that never prunes anything and keeps a complete record of every block
and transaction ever, including data that regular full nodes discard. Used by
services like block explorers (blockchain.com, mempool.space) that need to
answer queries like "show me transaction xyz from 2012".

---

## "Everyone Has a Copy" — What That Actually Means

When people say "the blockchain is decentralized and everyone has a copy", they
mean this precisely:

**Every full node holds an identical copy of the confirmed blockchain.**

Same blocks. Same transactions. Same order. If you and someone in Japan both run a
full node, your confirmed chains are bit-for-bit identical (assuming both are honest
and synced).

However — and this is what confused you when reading about the mempool — **the
unconfirmed transaction pool (mempool) differs between nodes**. This is expected,
normal, and not a problem.

```
                    CONFIRMED CHAIN (identical on all honest full nodes)
                    ┌─────────────────────────────────────────┐
Node A (Germany):   │ Block 0 → Block 1 → ... → Block 850,000 │   Mempool: [tx1, tx2, tx5]
                    └─────────────────────────────────────────┘

Node B (USA):       │ Block 0 → Block 1 → ... → Block 850,000 │   Mempool: [tx1, tx3, tx5, tx6]
                    └─────────────────────────────────────────┘

Node C (Japan):     │ Block 0 → Block 1 → ... → Block 850,000 │   Mempool: [tx2, tx4, tx5]
                    └─────────────────────────────────────────┘

Node D (Brazil):    │ Block 0 → Block 1 → ... → Block 850,000 │   Mempool: [tx1, tx2, tx3, tx4]
                    └─────────────────────────────────────────┘
```

The confirmed chain is the same. The mempools differ. This is fine because:
- The confirmed chain is permanent and has been agreed upon by the whole network
- The mempool is temporary and just a holding area — it does not need to match

---

## How a New Node Joins the Network — Step by Step

There is no registration. No account. No approval. Here is exactly what happens
when someone starts a brand new Bitcoin node:

### Step 1 — Install the Software

Download Bitcoin Core (or another implementation). Run it. It starts with zero
knowledge of the network.

### Step 2 — Find the First Peers (DNS Seeds)

The software has **hardcoded DNS seed addresses** — special domain names maintained
by trusted community members. When queried, these domains return a list of IP
addresses of currently active Bitcoin nodes.

```
seed.bitcoin.sipa.be         → [1.2.3.4, 5.6.7.8, 9.10.11.12, ...]
dnsseed.bluematt.me          → [13.14.15.16, 17.18.19.20, ...]
seed.bitcoinstats.com        → [...]
```

Your node picks a handful of these IPs and connects to them.

### Step 3 — Establish Peer Connections

Your node connects to peers via **TCP** (the same low-level protocol your browser
uses for HTTP). The handshake:

```
Your node  ──▶  "version" message (my version, my height, my IP)  ──▶  Peer
Your node  ◀──  "version" message (their version, their height)    ◀──  Peer
Your node  ──▶  "verack" (acknowledged)                            ──▶  Peer
Your node  ◀──  "verack" (acknowledged)                            ◀──  Peer
                     ↓
               Connected. Now exchange peer lists.
```

Your node asks peers for more peer addresses (`getaddr` / `addr` messages), building
up its own list of known nodes. Eventually it maintains ~8 **outbound connections**
(connections you initiated) and up to 125 **inbound connections** (others connecting
to you).

### Step 4 — Initial Block Download (IBD)

Your node has zero blocks. It needs to download and validate all ~850,000 blocks
ever created. This is IBD.

```
Your node: "getblocks" ──▶ Peer A
Peer A: "inv" [list of block hashes] ──▶ Your node
Your node: "getdata" [give me these blocks] ──▶ Peer A, Peer B, Peer C
Peers: [block data] ──▶ Your node
Your node: validate each block independently
Repeat until caught up to the current tip.
```

IBD takes **several hours to several days** depending on your hardware and internet
connection. During this time the node is not yet useful — it cannot relay transactions
or blocks because it has not verified history yet.

### Step 5 — Synced and Participating

Once IBD completes, your node is at the chain tip. It now:
- Receives new transactions and relays them to peers (gossip)
- Receives new blocks and validates + relays them
- Serves block data to other nodes doing IBD
- Maintains its own mempool
- Participates as a full, equal member of the network

---

## How Nodes Communicate — The P2P Protocol

There is no central server. Nodes communicate directly using a **custom protocol**
over TCP. Every node listens on port **8333** by default.

### Key Message Types

| Message | Direction | Purpose |
|---|---|---|
| `version` | both ways | Handshake — announce your version and chain height |
| `verack` | both ways | Acknowledge the version message |
| `inv` | both ways | "I have this tx/block" (sends the hash, not the data) |
| `getdata` | both ways | "Send me this tx/block" (in response to inv) |
| `tx` | both ways | Here is a full transaction |
| `block` | both ways | Here is a full block |
| `getaddr` | outbound | "Give me a list of peers you know" |
| `addr` | both ways | Here are peer IP addresses |
| `ping` | outbound | Are you still alive? |
| `pong` | inbound | Yes, still here |
| `reject` | both ways | I rejected your tx/block, here is why |

### How a Transaction Spreads (Gossip Propagation)

When you broadcast a transaction, it spreads through the network like a rumour:

```
Step 1: Your wallet sends the tx to Node A (your connected node)

Step 2: Node A validates it, adds to mempool, sends "inv" to its 8 peers
        (not the full tx — just the hash, to save bandwidth)

Step 3: Those 8 peers reply "getdata" if they don't have it yet
        Node A sends the full tx to each one that asked

Step 4: Each of those 8 peers validates it, adds to mempool,
        sends "inv" to their peers

Step 5: This continues outward like ripples in a pond

Result: Within 1-2 seconds, the transaction has reached thousands of nodes worldwide
```

```
                        YOU
                         │
                    ┌────▼────┐
                    │ Node A  │
                    └────┬────┘
          ┌──────────────┼──────────────┐
     ┌────▼────┐    ┌────▼────┐    ┌────▼────┐
     │ Node B  │    │ Node C  │    │ Node D  │
     └────┬────┘    └────┬────┘    └────┬────┘
    ┌─────┼──┐      ┌────┼───┐     ┌────┼───┐
   [B1] [B2][B3]  [C1][C2][C3]  [D1][D2][D3]
   ...continues outward in all directions...
```

The `inv` → `getdata` → `tx` dance avoids sending duplicate data. If Node B already
has the transaction (received it from another path), it ignores the `inv`.

---

## Why Nodes Can Have Different Mempools

Given the gossip propagation above, you might ask: if every transaction spreads to
every node, why do mempools differ?

### 1. Network Latency

A transaction does not reach all nodes at the exact same millisecond. During the
time it takes to propagate, new blocks may arrive, new transactions may be broadcast,
and the state of each node's mempool is slightly different at any given instant.

### 2. Different Memory Limits

Each node operator can configure how much RAM to dedicate to the mempool (default
300MB in Bitcoin Core). When a node's mempool is full, it evicts the lowest-fee
transactions. Node A might have more memory and keep transactions that Node B discarded.

### 3. Different Minimum Fee Policies

Node operators can set their own minimum fee rate. A node with `minrelayfee = 5 sat/vByte`
will reject and not relay transactions below that rate — those transactions may exist
on other nodes with lower thresholds.

### 4. Eviction Timing

Evictions happen independently on each node. A transaction may be evicted from
Node A's mempool during a congestion spike but still be alive in Node B's mempool.

### Why This Is Fine

The mempool is **not consensus-critical**. It is temporary storage. The only thing
that must match across all honest nodes is the **confirmed blockchain**. The mempool
is just an intermediate state that gets cleaned up every time a new block is mined.

---

## How the Confirmed Chain Stays Identical

If every node operates independently, how do they all end up with the same chain?

The answer is the **rules are identical and hardcoded**.

Every Bitcoin node runs the exact same validation logic:

```
For every block received:
  ✓ Does the block hash meet the current difficulty target?
  ✓ Is the block size within limits?
  ✓ Is the coinbase transaction the first transaction?
  ✓ Does the block correctly reference the previous block's hash?
  ✓ Is every transaction in the block valid?
     - Are all input UTXOs real and unspent?
     - Are all signatures valid?
     - No coins created from nothing?
  ✓ Is the merkle root correct?
  ✓ Is the timestamp within acceptable range?

If ALL pass → accept the block, add to chain
If ANY fail → reject the block entirely, do not relay it
```

Because the rules are identical on every node:
- A valid block is valid everywhere
- An invalid block is invalid everywhere
- No node can be tricked into accepting an invalid block

### The Longest Chain Rule

What if two nodes receive valid blocks in different orders and temporarily disagree
on which is the latest? The rule is simple:

> **The valid chain with the most cumulative proof of work wins.**

In practice: the longest chain (most blocks) wins. Nodes always switch to the longer
valid chain when they see one. Temporary disagreements resolve themselves within
one or two blocks — the moment one chain grows longer, all nodes switch to it.

### A Lying Node Gets Ignored

Suppose a malicious node sends you a fake block — one with invalid transactions
or a bad proof of work. Your node runs the validation checklist, the block fails,
and your node:
- Rejects the block
- Does not relay it to peers (so the lie stops propagating)
- May disconnect from that peer and ban their IP

A malicious node cannot force you to accept invalid data. You check everything yourself.

---

## What Happens When You Send a Transaction — The Full Node Perspective

```
1. Your wallet builds and signs the transaction

2. Your wallet sends it to your connected node (or directly to one you trust)

3. That node runs the full validation checklist:
   ✓ Valid format?
   ✓ Inputs exist and are unspent?
   ✓ Signatures valid?
   ✓ Fee rate above minimum?
   → PASS: add to mempool

4. That node sends "inv" (the tx hash) to all its peers

5. Peers that don't have it reply "getdata"

6. The tx is sent to them, they validate and add to their mempools

7. Gossip continues — within ~2 seconds it has reached most of the network

8. A miner's node receives it, adds it to the mempool

9. Miner includes it in the next block template (based on fee rate)

10. Miner finds a valid block and broadcasts it

11. The block propagates to all nodes via the same gossip mechanism

12. Every node validates the block, confirms the transaction,
    removes it from their mempool, and updates their UTXO set

13. Your wallet detects the transaction in the new block
    → "1 confirmation"
```

---

## Nodes in RustyBlocks

Right now, `RustyBlocks` is a **single-node implementation**. There is no networking
code — just the block and blockchain data structures, and the merkle tree logic.

This is the right way to start. Get the core data structures correct first, then
add networking.

When the time comes to add P2P, here is what needs to be built:

```rust
// What you will eventually need:

struct Node {
    blockchain: BlockChain,        // the confirmed chain
    mempool: Mempool,              // unconfirmed transactions
    peers: Vec<PeerConnection>,    // active TCP connections
    known_addresses: Vec<SocketAddr>, // peer IPs we know about
}

// Core networking tasks:
// 1. TCP listener — accept inbound peer connections
// 2. Outbound connector — connect to known peers
// 3. Message serializer — encode/decode P2P messages as bytes
// 4. Message handler — route incoming messages (inv, tx, block, etc.)
// 5. IBD logic — request and validate historical blocks on first start
```

This maps to **Phase 7** of the project roadmap (P2P Networking).

---

## Summary

```
A node = any computer running Bitcoin software, connected to peers.
No owner. No permission. No central server.
The network IS the collection of all nodes.

Types:
  Full node     → validates everything, most trusted
  Pruned node   → validates everything, saves disk space
  Mining node   → full node + mining hardware
  Light / SPV   → trusts peers, used in mobile wallets
  Archival node → keeps all history, used by block explorers

"Everyone has a copy" means:
  → Confirmed blockchain: identical on all honest full nodes ✓
  → Mempool: differs between nodes — this is normal and fine ✓

Joining the network:
  Install software → DNS seeds → connect to peers → IBD → synced → participating

Communication:
  Direct TCP between peers. Gossip protocol spreads txs and blocks.
  A transaction reaches the whole network in ~2 seconds.

Chain stays identical because:
  → Same hardcoded rules on every node
  → Invalid blocks are rejected independently by everyone
  → Longest valid chain wins
  → Lying nodes are ignored — you verify everything yourself
```
