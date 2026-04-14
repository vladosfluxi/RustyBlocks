# Bitcoin Mining — A Beginner's Guide

---

## Abbreviations

| Abbreviation | Stands For |
|---|---|
| PoW | Proof of Work |
| nonce | Number Used Once |
| hash | The fixed-size output of a hash function (SHA-256 in Bitcoin) |
| target | The 256-bit number your hash must be less than to win |
| difficulty | A human-readable measure of how hard it is to find a valid hash |
| coinbase tx | The special first transaction in every block that pays the miner |
| UTXO | Unspent Transaction Output |
| PoW | Proof of Work — evidence that you burned real computational effort |
| orphan block | A valid block that lost the race and was not added to the main chain |

---

## What is Mining?

Mining is a **competition**.

Every ~10 minutes, thousands of computers around the world race to solve a
mathematical puzzle. The first one to solve it wins the right to add the next
block to the blockchain — and collects the block reward as payment.

The puzzle is not clever math. It is pure **brute force guessing**.
There is no shortcut. You just have to try billions of numbers until one works.

This is called **Proof of Work** — your winning hash is *proof* that you did
an enormous amount of computation to find it.

---

## The Hash Puzzle

SHA-256 takes any input and produces a 256-bit output that looks completely random:

```
SHA-256("hello")  = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b982
SHA-256("hello1") = 91e9240f415223982edc345532630710e94a7f52cd5f48f5ee1afc555078f0ab
SHA-256("hello2") = 87298cc2f31fba73181ea2a9e6ef10dce21ed95e98bdac9c4e1504ea16f486e4
```

Changing even one character produces a completely different output.
You cannot look at a hash and work backwards to find the input — it is a one-way function.

**The puzzle**: find an input whose hash, when read as a number, is **less than the target**.

### What does "too big" mean?

A SHA-256 hash is 32 bytes — 256 bits. You can treat those 32 bytes as one enormous
integer. For example:

```
hash bytes:  a3 f8 2c 1d ...
as a number: very large — starts with 0xa3 which is 163 in decimal
```

The target is also a 256-bit number. Comparing them is just comparing two integers:

```
hash   = 0xa3f82c1d...   (163 × 2^248 + ...)   ← enormous number
target = 0x0000ffff...   (65535 × 2^232 + ...)  ← much smaller number

hash > target  →  LOSE  ("too big")
hash < target  →  WIN
```

So "too big" simply means: **your hash, treated as a number, is larger than the target**.
The vast majority of all possible hashes are larger than the target — that is what makes
the puzzle hard.

Visually, a hash that starts with large hex digits (like `a3`, `ff`, `c1`) is a big number.
A hash that starts with zeros (like `0000`, `00000000`) is a tiny number — and tiny is what you need:

```
target:
0000ffff00000000000000000000000000000000000000000000000000000000

a losing hash  (too big — starts with a3, which is > 00):
a3f82c1d9b0e4f7a2c8d1e5b9f3a6c0d4e8b2f7a1c5d9e3b7f0a4c8d2e6b1f

another losing hash  (too big — starts with 0001, which is > 0000):
0001af3b9c0e4f7a2c8d1e5b9f3a6c0d4e8b2f7a1c5d9e3b7f0a4c8d2e6b1f

a winning hash  (starts with 0000 and the next bytes are also smaller):
00000000000000000a3f82c1d9b0e4f7a2c8d1e5b9f3a6c0d4e8b2f7a1c5d9e
```

More leading zeros = smaller number = harder puzzle.

---

## The Nonce — Your Only Variable

A block header contains:

```
index | prev_hash | merkle_root | timestamp | difficulty | nonce
```

Almost everything is fixed. You cannot change the transactions (that would change
the merkle root). You cannot change the previous hash. You cannot fake the timestamp.

The **nonce** is the one field you are free to change.

You try every possible value, recomputing the hash each time, until you find one
that beats the target:

```
┌─────────────────────────────────────────────────────────────────┐
│  MINING LOOP                                                    │
│                                                                 │
│  nonce = 0                                                      │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  hash = SHA256(SHA256(header with current nonce))       │   │
│  │                                                         │   │
│  │  if hash < target  ──▶  YOU WIN! Broadcast the block.  │   │
│  │                                                         │   │
│  │  else  ──▶  nonce += 1  ──▶  try again                 │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│  nonce=0        → 9f3a...  too big                             │
│  nonce=1        → c1b8...  too big                             │
│  nonce=2        → 7e2d...  too big                             │
│  ...                                                            │
│  nonce=3,918,203 → 00000000a3f8...  ✓ WINNER                  │
└─────────────────────────────────────────────────────────────────┘
```

A nonce is a `u32` — it can hold ~4 billion values. If all 4 billion are exhausted
without finding a valid hash, miners also change the timestamp or the order of
transactions to get a fresh set of hashes to try.

---

## Difficulty and Target — Where Does the Target Come From?

This is the most important thing to understand about mining.

### The maximum possible target (easiest setting)

The largest valid target in Bitcoin is hardcoded as:

```
0x00000000FFFF0000000000000000000000000000000000000000000000000000
```

This is called the **genesis target** or **difficulty-1 target**. It is the easiest
the puzzle can ever be. Any hash starting with 8 zero hex digits (4 zero bytes) would
win at this setting.

### How the target shrinks over time

The network wants one block every 10 minutes on average. But more miners keep joining,
making the puzzle get solved faster. So Bitcoin recalculates the target every **2016
blocks** (~2 weeks) to compensate.

The formula is:

```
new_target = old_target × (actual time to mine last 2016 blocks / expected time)
expected time = 2016 blocks × 10 minutes = 20160 minutes
```

Example:
- Last 2016 blocks took only 10080 minutes (twice as fast as expected)
- `new_target = old_target × (10080 / 20160) = old_target × 0.5`
- The target is cut in half → twice as hard to find a valid hash → blocks slow back down

Example in the other direction:
- Last 2016 blocks took 40320 minutes (twice as slow as expected)
- `new_target = old_target × (40320 / 20160) = old_target × 2`
- The target doubles → twice as easy → blocks speed back up

Bitcoin caps the adjustment at a factor of 4 in either direction per period,
to prevent wild swings.

### Difficulty is just a human-readable label

**Difficulty** is not stored in the block as the raw target — it is a derived number
for humans to read:

```
difficulty = genesis_target / current_target
```

When difficulty = 1, target = genesis_target (easiest).
When difficulty = 1,000,000, target = genesis_target / 1,000,000 (one million times harder).

Bitcoin's difficulty today is around **90 trillion** — meaning your hash must be
90 trillion times smaller than the genesis target to win.

### Where the target is actually stored in a block

Bitcoin stores it in a compressed 4-byte format called **nBits** in the block header.
It encodes the target approximately, not exactly. When a node validates a block, it
unpacks nBits back into the full 256-bit target and checks: `hash < target`.

### In your RustyBlocks implementation

Your `difficulty: u8` is a simplified version. The way to use it is:

```
difficulty = N  means  the first N bytes of the hash must be 0x00
```

So the target is implicitly:

```
difficulty = 1  →  target = 0x00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
difficulty = 2  →  target = 0x0000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
difficulty = 3  →  target = 0x000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
```

Each extra zero byte makes it ~256x harder (since each byte has 256 possible values,
and only 0x00 passes).

Bitcoin adjusts difficulty **every 2016 blocks** (roughly every 2 weeks):

- If the last 2016 blocks were mined faster than 10 min/block average → difficulty goes **up**
- If slower → difficulty goes **down**

This keeps the average block time locked at **~10 minutes** regardless of how much
computing power joins or leaves the network.

---

## Where the Target Comes From — No Broadcasting Needed

The target is **never sent over the network**. Every node calculates it independently
from the block history it already has.

```
You join the network
        ↓
You download all previous blocks
        ↓
You look at the timestamps of the last 2016 blocks
        ↓
You apply the adjustment formula yourself
        ↓
You arrive at the current target

Every other honest node does the exact same steps
        ↓
Everyone gets the exact same target
```

This works because the calculation is **deterministic** — same block history, same
formula, same result. No server, no coordinator, no trust required.

### What gets broadcast and what does not

```
NEVER broadcast:  your individual nonce guesses
                  (you try billions per second — broadcasting them would be absurd)

NEVER broadcast:  the target
                  (everyone calculates it themselves)

ONLY broadcast:   a complete finished block, after you already won
```

The mining process is entirely **silent** until you win:

```
1. Calculate the target locally (from chain history)

2. Mine silently on your own machine
   Try nonce=0, nonce=1, nonce=2 ... telling nobody

3. Find a winning nonce

4. NOW broadcast the full block to the network
   (header + all transactions inside)

5. Every node receives it, recalculates the target themselves,
   and checks: hash < target? → yes → block accepted
```

### Why this works without any coordination

The rules are hardcoded identically in every Bitcoin node:

- "Genesis target is this exact hardcoded value"
- "Recalculate every 2016 blocks"
- "Use this exact formula"
- "Cap adjustment at 4x"

The blockchain itself is the shared source of truth. Everyone reads from it
independently and reaches the same conclusion.

---

## Comparing Hashes — No Decimal Needed

You never convert a hash to decimal. Comparing two `[u8; 32]` arrays is enough.

A 32-byte array read left to right is a big-endian 256-bit integer. Comparing two
of them byte by byte, from left to right, is identical to comparing them as integers.
The moment one byte differs, the one with the smaller byte at that position is the
smaller number — and you stop there.

```
hash   = [0x00, 0x00, 0x00, 0x00, 0xa3, 0xf8, ...]
target = [0x00, 0x00, 0x00, 0x00, 0xff, 0x12, ...]

byte 0: 0x00 == 0x00  → tie, keep going
byte 1: 0x00 == 0x00  → tie, keep going
byte 2: 0x00 == 0x00  → tie, keep going
byte 3: 0x00 == 0x00  → tie, keep going
byte 4: 0xa3 <  0xff  → hash is smaller → WIN ✓
```

Another example — losing hash:

```
hash   = [0x00, 0x00, 0x01, 0x00, ...]
target = [0x00, 0x00, 0x00, 0xff, ...]

byte 0: 0x00 == 0x00  → tie, keep going
byte 1: 0x00 == 0x00  → tie, keep going
byte 2: 0x01 >  0x00  → hash is bigger → LOSE ✗
```

In Rust, `[u8; 32]` implements `PartialOrd` using exactly this lexicographic
(left-to-right) comparison, so you can write:

```rust
if hash < target {
    // winner
}
```

No decimal. No conversion. Just a direct array comparison.

---

## What Happens When You Win

1. **You find a nonce** that produces a hash below the target.

2. **You immediately broadcast** your block to all peers on the network.

3. **Every node independently verifies**:
   - Does `double_hash(header)` actually equal the block's hash? ✓
   - Is that hash below the current target? ✓
   - Is `prev_hash` pointing to the current chain tip? ✓
   - Are all transactions valid (no double spends, valid signatures)? ✓
   - Is the merkle root correct? ✓

4. **Nodes accept the block** and add it to their chain. They immediately
   start mining the *next* block on top of yours.

5. **You get paid** via the coinbase transaction — the first transaction in your
   block, which you wrote yourself, sending the block reward to your own address.
   Currently **3.125 BTC** per block.

6. **The reward is locked** for 100 blocks (the maturity rule). You cannot spend
   your coinbase output until 100 more blocks have been built on top of yours.
   This protects against orphan scenarios (see below).

---

## Who Gets the Reward?

**Only the miner who found the winning nonce.** Everyone else gets nothing for
that round — all their work is discarded. It is winner-takes-all.

In practice, solo mining is like buying a single lottery ticket.
Most miners join **mining pools**:

- Every miner in the pool works on the same block template
- They submit **partial solutions** (hashes that are "almost good enough") to the
  pool server as proof of work done — these are called **shares**
- When any pool member finds the actual winning hash, the pool reward is split
  among all members proportionally to how many shares they submitted
- The pool operator takes a small fee (1–3%)

This converts the lottery into a steady, predictable income stream.

---

## Orphan Blocks — When Two Miners Win at the Same Time

Occasionally, two miners find a valid block at almost the same moment and
broadcast them simultaneously. The network temporarily forks:

```
         ... ── Block 100 ── Block 101A  ← half the network builds here
                          └─ Block 101B  ← other half builds here
```

Both 101A and 101B are valid. The conflict resolves naturally:

- Whichever chain grows longer first wins
- Nodes switch to the longer chain
- The block on the shorter chain is **orphaned** — dropped from the main chain
- The miner of the orphaned block receives **no reward**
- Transactions in the orphaned block go back to the mempool

This is why exchanges and merchants wait for **6 confirmations** before treating
a payment as final — 6 blocks deep makes a reversal statistically impossible.

---

## Why Mining Secures the Blockchain

Suppose an attacker wants to rewrite history — change a transaction in Block 100.

1. They must re-mine Block 100 with the altered transaction (find a new valid nonce)
2. That changes Block 100's hash → Block 101's `prev_hash` is now wrong → must re-mine 101
3. Must re-mine 102, 103, 104... every block after the target
4. **AND** they must do all of this faster than the honest network is adding new blocks

At Bitcoin's current scale, this would require more electricity than most countries
consume. It is economically and physically impossible for any realistic attacker.

This is the security guarantee of Proof of Work.

---

## Relation to RustyBlocks Code

Your current `BlockHead` in `src/block.rs`:

```rust
struct BlockHead {
    index: u32,
    merkle_root_hash: [u8; 32],
    hash: [u8; 32],           // the winning hash you found
    hash_prev: [u8; 32],      // previous block's hash
    difficulty: u8,            // how many leading zero bytes required
    timestamp: u64,
}
```

To implement mining, you need to add one field:

```rust
nonce: u64,   // the number you increment until hash < target
```

Then implement a `mine()` function that:
1.2 Serializes the header into bytes
2. Double-hashes it
3. Checks if the result starts with `difficulty` zero bytes
4. If not — increments `nonce` and tries again
5. If yes — stores the winning hash in `hash` and returns

---

## Summary

```
Mining = repeatedly hashing the block header with different nonces
         until the hash is small enough (has enough leading zeros).

The first miner to find it wins the block reward.
The network adjusts difficulty every 2016 blocks to keep
the average time per block at ~10 minutes.

Winning hash  →  broadcast block  →  network verifies  →  reward paid via coinbase tx
Losing block  →  orphaned  →  no reward
```
