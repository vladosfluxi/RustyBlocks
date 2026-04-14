# UTXO, Inputs, Outputs — Complete Structure Guide

---

## Abbreviations

| Abbreviation | Stands For |
|---|---|
| UTXO | Unspent Transaction Output — a coin sitting on the blockchain waiting to be spent |
| input | The part of a transaction that consumes an existing UTXO |
| output | The part of a transaction that creates a new UTXO |
| scriptSig | The unlocking proof placed in an input (signature + public key) |
| scriptPubKey | The locking script placed in an output (the address/conditions) |
| txid | Transaction ID — the double-SHA256 hash of an entire transaction |
| vout | The index (0, 1, 2...) of a specific output inside a transaction |
| coinbase | The special first transaction in every block — creates new coins from nothing |
| fee | The difference between total input value and total output value — goes to the miner |
| UTXO set | The complete list of all unspent outputs on the blockchain — maintained by every node |

---

## The Core Rule — Burned Into Your Brain

```
  ┌─────────────────────────────────────────────────────────────┐
  │                                                             │
  │  A transaction DESTROYS old UTXOs (inputs)                  │
  │              and CREATES new UTXOs (outputs)                │
  │                                                             │
  │  Nothing is edited. Nothing is updated.                     │
  │  Old ones die. New ones are born.                           │
  │  The blockchain only ever grows — never changes.            │
  │                                                             │
  └─────────────────────────────────────────────────────────────┘
```

---

## What a UTXO Is

A UTXO is a single unspent output sitting on the blockchain.
It has exactly two pieces of information:

```
  ┌──────────────────────────────────────────────────────────┐
  │  UTXO                                                    │
  │                                                          │
  │  value      →  how many satoshis this output holds       │
  │  scriptPubKey → the lock (who is allowed to spend this)  │
  └──────────────────────────────────────────────────────────┘
```

The UTXO set is the master list every full node keeps in memory:

```
  UTXO SET (snapshot):
  ┌─────────────────┬───────┬──────────────────────────────────┐
  │ txid            │ vout  │ output                           │
  ├─────────────────┼───────┼──────────────────────────────────┤
  │ abc123...       │   0   │ 0.5 BTC  → Alice's address       │
  │ abc123...       │   1   │ 0.3 BTC  → Bob's address         │
  │ def456...       │   0   │ 1.2 BTC  → Carol's address       │
  │ ghi789...       │   0   │ 0.05 BTC → Dave's address        │
  │ ...             │  ...  │ ...                              │
  └─────────────────┴───────┴──────────────────────────────────┘

  The pair (txid + vout) uniquely identifies ONE specific UTXO
  in the entire history of the blockchain.
```

---

## Full Transaction Structure

A transaction is a package with four parts:

```
  TRANSACTION
  ┌──────────────────────────────────────────────────────────────┐
  │  version       u32    which rule set applies (1 or 2)        │
  │                                                              │
  │  inputs[]      list of UTXOs being consumed                  │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  INPUT                                                 │  │
  │  │  prev_txid    [u8; 32]  which past transaction         │  │
  │  │  prev_vout    u32       which output inside that tx    │  │
  │  │  scriptSig    Vec<u8>   the unlocking proof            │  │
  │  │  sequence     u32       timelock / RBF flag            │  │
  │  └────────────────────────────────────────────────────────┘  │
  │  (can have multiple inputs — one per UTXO being spent)       │
  │                                                              │
  │  outputs[]     list of new UTXOs being created               │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  OUTPUT                                                │  │
  │  │  value        u64      how many satoshis               │  │
  │  │  scriptPubKey Vec<u8>  the locking script (the address)│  │
  │  └────────────────────────────────────────────────────────┘  │
  │  (can have multiple outputs — recipient + change + more)     │
  │                                                              │
  │  locktime      u32    earliest block/time this tx is valid   │
  └──────────────────────────────────────────────────────────────┘
```

---

## prev_txid and prev_vout — What They Actually Point At

This is the most important thing to understand about inputs.

`prev_txid` is the txid of whichever **past transaction created** the UTXO
you want to spend. `prev_vout` is the exact slot number inside that transaction.

Concrete example — two transactions exist in history:

```
  TRANSACTION A  (txid: "aaaa")
  ┌──────────────────────────────────────────────────────────┐
  │  outputs:                                                │
  │    vout 0  →  1.0 BTC  →  Bob's address                  │
  │    vout 1  →  0.5 BTC  →  Carol's address                │
  └──────────────────────────────────────────────────────────┘

  TRANSACTION B  (txid: "bbbb")
  ┌──────────────────────────────────────────────────────────┐
  │  outputs:                                                │
  │    vout 0  →  2.0 BTC  →  Bob's address                  │
  └──────────────────────────────────────────────────────────┘
```

Bob wants to spend BOTH of his UTXOs at once.
He builds a new transaction with two inputs, each pointing at a different
past output:

```
  TRANSACTION C  (Bob spending everything he has)
  ┌──────────────────────────────────────────────────────────┐
  │  inputs:                                                 │
  │                                                          │
  │    INPUT 0:                                              │
  │      prev_txid: "aaaa"  ← points to Transaction A       │
  │      prev_vout: 0       ← slot 0 inside A  (1.0 BTC)    │
  │      scriptSig: <Bob's proof>                            │
  │                         ↑                               │
  │                         └── consumes 1.0 BTC            │
  │                                                          │
  │    INPUT 1:                                              │
  │      prev_txid: "bbbb"  ← points to Transaction B       │
  │      prev_vout: 0       ← slot 0 inside B  (2.0 BTC)    │
  │      scriptSig: <Bob's proof>                            │
  │                         ↑                               │
  │                         └── consumes 2.0 BTC            │
  │                                                          │
  │  total consumed:  3.0 BTC                                │
  │                                                          │
  │  outputs:                                                │
  │    vout 0  →  2.9 BTC  →  Alice's address               │
  │    vout 1  →  0.09 BTC →  Bob's change address           │
  │    fee:       0.01 BTC    (implicit, goes to miner)      │
  └──────────────────────────────────────────────────────────┘
```

Carol's UTXO (Transaction A, vout 1) is untouched — Bob only pointed at
vout 0 of Transaction A. Without `prev_vout`, the node would not know
which of the two outputs Bob is claiming.

```
  (prev_txid, prev_vout) together = a precise address
  into the entire history of the blockchain pointing at
  exactly ONE specific UTXO — nothing else.
```

---

## INPUT — In Full Detail

An input says: **"I want to spend this specific existing UTXO."**

```
  ┌──────────────────────────────────────────────────────────────┐
  │  INPUT                                                       │
  │                                                              │
  │  prev_txid: [u8; 32]                                         │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  The txid of the transaction that CREATED the UTXO     │  │
  │  │  you want to spend.                                    │  │
  │  │                                                        │  │
  │  │  Example:                                              │  │
  │  │  a3f8c1d9b0e4f7a2c8d1e5b9f3a6c0d4...  (32 bytes)      │  │
  │  │                                                        │  │
  │  │  This is a pointer into blockchain history.            │  │
  │  │  "Go find transaction with this hash."                 │  │
  │  └────────────────────────────────────────────────────────┘  │
  │                                                              │
  │  prev_vout: u32                                              │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  The index of the specific output inside that tx.      │  │
  │  │                                                        │  │
  │  │  Why needed: one transaction can have many outputs.    │  │
  │  │  vout = 0 means the first output.                      │  │
  │  │  vout = 1 means the second output.                     │  │
  │  │  etc.                                                  │  │
  │  │                                                        │  │
  │  │  prev_txid + prev_vout together = unique pointer       │  │
  │  │  to exactly ONE UTXO in the entire blockchain.         │  │
  │  └────────────────────────────────────────────────────────┘  │
  │                                                              │
  │  scriptSig: Vec<u8>                                          │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  The PROOF that you are allowed to spend this UTXO.    │  │
  │  │                                                        │  │
  │  │  For a standard P2PKH transaction it contains:         │  │
  │  │    1. signature (sig_r, sig_s) encoded in DER (~71 B)  │  │
  │  │    2. your public key (compressed, 33 bytes)           │  │
  │  │                                                        │  │
  │  │  The private key is NEVER here.                        │  │
  │  │  It was used to produce the signature and then         │  │
  │  │  stayed in your wallet forever.                        │  │
  │  └────────────────────────────────────────────────────────┘  │
  │                                                              │
  │  sequence: u32                                               │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  Default: 0xFFFFFFFF  (no special meaning)              │  │
  │  │  Used for: relative timelocks, RBF signalling          │  │
  │  │  You can ignore this for a basic implementation.       │  │
  │  └────────────────────────────────────────────────────────┘  │
  └──────────────────────────────────────────────────────────────┘
```

---

## OUTPUT — In Full Detail

An output says: **"I am creating a new UTXO. Here is how much it holds and who can spend it."**

```
  ┌──────────────────────────────────────────────────────────────┐
  │  OUTPUT                                                      │
  │                                                              │
  │  value: u64                                                  │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  The amount in SATOSHIS (not BTC — always integers).   │  │
  │  │                                                        │  │
  │  │  1 BTC = 100,000,000 satoshis                          │  │
  │  │                                                        │  │
  │  │  u64 because Bitcoin's max supply is                   │  │
  │  │  2,100,000,000,000,000 satoshis — fits in u64.         │  │
  │  │  Never use floats for money. Ever.                     │  │
  │  └────────────────────────────────────────────────────────┘  │
  │                                                              │
  │  scriptPubKey: Vec<u8>                                       │
  │  ┌────────────────────────────────────────────────────────┐  │
  │  │  The LOCK on this output.                              │  │
  │  │  Defines who is allowed to spend it in the future.    │  │
  │  │                                                        │  │
  │  │  For a standard P2PKH transaction it contains:         │  │
  │  │    OP_DUP                                              │  │
  │  │    OP_HASH160                                          │  │
  │  │    <recipient's pubkey hash>  ← the address, 20 bytes  │  │
  │  │    OP_EQUALVERIFY                                      │  │
  │  │    OP_CHECKSIG                                         │  │
  │  │                                                        │  │
  │  │  In plain English:                                     │  │
  │  │  "Whoever can prove they own this address can spend"   │  │
  │  │                                                        │  │
  │  │  This is written by the SENDER using the              │  │
  │  │  RECEIVER'S address. The receiver never touches it.   │  │
  │  └────────────────────────────────────────────────────────┘  │
  └──────────────────────────────────────────────────────────────┘
```

---

## A Real Transaction — All Fields Filled In

Bob sends Alice 0.3 BTC. Bob has one UTXO of 0.5 BTC.

```
  ┌──────────────────────────────────────────────────────────────┐
  │  TRANSACTION                                                 │
  │                                                              │
  │  version:    1                                               │
  │                                                              │
  │  inputs: [                                                   │
  │    INPUT {                                                   │
  │      prev_txid:  a3f8c1d9b0e4f7...  ← Bob's old tx          │
  │      prev_vout:  0                  ← first output of that tx│
  │      scriptSig:  <sig_r><sig_s>     ← Bob's signature        │
  │                  <Bob's pubkey>     ← Bob's public key       │
  │      sequence:   0xFFFFFFFF                                  │
  │    }                                                         │
  │  ]                                                           │
  │                                                              │
  │  outputs: [                                                  │
  │    OUTPUT {              ← vout 0 (Alice gets paid)          │
  │      value:       30_000_000        ← 0.3 BTC in satoshis    │
  │      scriptPubKey: OP_DUP           ← locked to Alice        │
  │                    OP_HASH160                                │
  │                    <Alice's pubkey hash>                     │
  │                    OP_EQUALVERIFY                            │
  │                    OP_CHECKSIG                               │
  │    },                                                        │
  │    OUTPUT {              ← vout 1 (Bob's change)             │
  │      value:       19_000_000        ← 0.19 BTC in satoshis   │
  │      scriptPubKey: OP_DUP           ← locked to Bob          │
  │                    OP_HASH160                                │
  │                    <Bob's pubkey hash>                       │
  │                    OP_EQUALVERIFY                            │
  │                    OP_CHECKSIG                               │
  │    }                                                         │
  │  ]                                                           │
  │                                                              │
  │  locktime:   0                ← valid immediately            │
  │                                                              │
  │  fee (implicit):  50_000_000 - 30_000_000 - 19_000_000      │
  │                 = 1_000_000 satoshis  (0.01 BTC)             │
  │                   goes to the miner, not an explicit output  │
  └──────────────────────────────────────────────────────────────┘
```

---

## What Happens to the UTXO Set

```
  BEFORE:
  ┌──────────────────────────────────────────────────────────┐
  │  UTXO SET                                                │
  │  (a3f8c1..., vout 0)  →  0.5 BTC  locked to Bob  ✓      │
  └──────────────────────────────────────────────────────────┘

  TRANSACTION IS CONFIRMED IN A BLOCK:

  AFTER:
  ┌──────────────────────────────────────────────────────────┐
  │  UTXO SET                                                │
  │  (a3f8c1..., vout 0)  →  DESTROYED  ✗  (was Bob's)      │
  │  (new_txid, vout 0)   →  0.30 BTC   ✓  (Alice's)        │
  │  (new_txid, vout 1)   →  0.19 BTC   ✓  (Bob's change)   │
  └──────────────────────────────────────────────────────────┘

  The old UTXO did not get its address changed.
  It was destroyed and two brand new UTXOs were created.
  Nothing was edited. Only additions.
```

---

## The Coinbase Transaction — Different Rules

Every block has one special transaction at the top: the **coinbase**.
It has no inputs (no prev_txid — there is nothing to consume).
It creates new coins from nothing.

```
  ┌──────────────────────────────────────────────────────────────┐
  │  COINBASE TRANSACTION                                        │
  │                                                              │
  │  version:    1                                               │
  │                                                              │
  │  inputs: [                                                   │
  │    INPUT {                                                   │
  │      prev_txid:  0000000000000000...  ← all zeros (no prev)  │
  │      prev_vout:  0xFFFFFFFF           ← special marker       │
  │      scriptSig:  <arbitrary data>     ← miner puts anything  │
  │                  (Satoshi put a newspaper headline here      │
  │                   in the genesis block)                      │
  │      sequence:   0xFFFFFFFF                                  │
  │    }                                                         │
  │  ]                                                           │
  │                                                              │
  │  outputs: [                                                  │
  │    OUTPUT {                                                  │
  │      value: block_reward + all_fees_from_block               │
  │             currently: 312_500_000 + collected fees          │
  │             (3.125 BTC + fees)                               │
  │      scriptPubKey: locked to MINER'S own address            │
  │    }                                                         │
  │  ]                                                           │
  │                                                              │
  │  locktime:   0                                               │
  └──────────────────────────────────────────────────────────────┘

  The coinbase output cannot be spent until 100 more blocks
  are built on top (maturity rule).
```

---

## Rust Structs for Your RustyBlocks Code

Based on everything above, here is what your structs should look like:

```rust
struct Transaction {
    version:  u32,
    inputs:   Vec<TxInput>,
    outputs:  Vec<TxOutput>,
    locktime: u32,
}

struct TxInput {
    prev_txid:  [u8; 32],   // which past transaction
    prev_vout:  u32,        // which output inside that tx
    script_sig: Vec<u8>,    // signature + public key (unlocking proof)
    sequence:   u32,        // default 0xFFFFFFFF
}

struct TxOutput {
    value:          u64,      // amount in satoshis — NEVER use floats
    script_pubkey:  Vec<u8>,  // locking script (contains recipient's address)
}
```

And the UTXO set — how nodes track what is spendable:

```rust
use std::collections::HashMap;

// key = (txid, vout) — unique pointer to one UTXO
// value = the output itself
type UtxoSet = HashMap<([u8; 32], u32), TxOutput>;
```

When a block is confirmed, you update the UTXO set like this:

```rust
fn apply_block(utxo_set: &mut UtxoSet, transactions: &[Transaction]) {
    for tx in transactions {

        // DESTROY the inputs (mark them as spent)
        for input in &tx.inputs {
            utxo_set.remove(&(input.prev_txid, input.prev_vout));
        }

        // CREATE the new outputs
        let txid = compute_txid(tx);
        for (index, output) in tx.outputs.iter().enumerate() {
            utxo_set.insert((txid, index as u32), output.clone());
        }
    }
}
```

---

## Every Field — Quick Reference Card

```
  TRANSACTION
  ├── version       u32        1 = original, 2 = relative timelocks
  ├── inputs[]
  │   ├── prev_txid [u8; 32]   hash of the tx that created the UTXO being spent
  │   ├── prev_vout u32        index of the output inside that tx (0, 1, 2...)
  │   ├── script_sig Vec<u8>   signature (sig_r + sig_s) + public key
  │   └── sequence  u32        0xFFFFFFFF = default, no special meaning
  ├── outputs[]
  │   ├── value     u64        amount in satoshis (1 BTC = 100_000_000)
  │   └── script_pubkey Vec<u8> locking script — contains recipient's address
  └── locktime      u32        0 = valid now, else block height or timestamp

  FEE (not a field — it is implicit):
  fee = sum(all input values) - sum(all output values)
  goes to the miner via their coinbase transaction
  never stored explicitly anywhere in the transaction

  TXID (not a field — it is derived):
  txid = double_sha256(serialized transaction bytes)
  computed by anyone who has the transaction
  used by future inputs to reference this transaction
```

---

## Summary

```
  UTXO = a coin sitting on the blockchain
         value (satoshis) + scriptPubKey (who can spend it)
         identified by (txid, vout) — a unique pointer

  INPUT  = "I want to destroy this UTXO"
           contains (prev_txid + prev_vout) to point at it
           contains scriptSig to prove permission
           private key NEVER appears here

  OUTPUT = "Create a new UTXO with this value locked to this address"
           contains value in satoshis
           contains scriptPubKey (the lock — written by sender
           using receiver's address)

  TRANSACTION = destroys inputs, creates outputs
                fee = input total - output total (implicit, goes to miner)
                nothing is edited — old UTXOs die, new ones are born

  COINBASE = special first tx in every block
             no inputs — creates coins from nothing
             miner pays themselves block_reward + all fees
```
