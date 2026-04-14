# Bitcoin Transactions — A Beginner's Guide

---

## Abbreviations

| Abbreviation | Stands For |
|---|---|
| TX / TxID | Transaction / Transaction ID |
| UTXO | Unspent Transaction Output |
| BTC | Bitcoin |
| sat / sats | Satoshi / Satoshis (smallest unit of BTC) |
| ECDSA | Elliptic Curve Digital Signature Algorithm |
| vout | Vector Output (index of an output inside a transaction) |
| scriptSig | Script Signature (the unlocking script placed by the spender) |
| scriptPubKey | Script Public Key (the locking script placed by the receiver) |
| P2PKH | Pay To Public Key Hash |
| SHA-256 | Secure Hash Algorithm 256-bit |

---

## What is a Transaction?

A Bitcoin transaction is a **signed message** that says:

> "I want to move coins from address A to address B."

It is not like a bank transfer where the bank updates a balance in a database.
Bitcoin has **no balances**. Instead, it has a list of **unspent outputs** (UTXOs).
Every transaction **consumes** existing UTXOs and **creates** new ones.

Think of UTXOs like physical coins or banknotes. You cannot split a banknote without
going to a cashier and exchanging it. In Bitcoin, your transaction is that exchange.

---

## The UTXO Model — The Core Concept

Imagine you have a 10 BTC banknote and you want to pay someone 3 BTC.

1. You hand over the 10 BTC banknote (this is your **input**)
2. The cashier (the network) creates:
   - A 3 BTC note for the recipient (**output 0**)
   - A 7 BTC note back to you as change (**output 1**)

The original 10 BTC note is now **spent** and destroyed.
The two new notes are now **unspent** (UTXOs) — waiting to be spent by their owners.

```
┌─────────────────────────────────────────────────────┐
│                   TRANSACTION                       │
│                                                     │
│  INPUT                        OUTPUTS               │
│  ┌──────────────────┐         ┌─────────────────┐   │
│  │ prev_txid: abc.. │────────▶│ value: 3 BTC    │   │
│  │ prev_vout: 0     │    │    │ to: Alice       │   │
│  │ (your 10 BTC)    │    │    └─────────────────┘   │
│  └──────────────────┘    │    ┌─────────────────┐   │
│                           └──▶│ value: 6.9 BTC  │   │
│                               │ to: You (change)│   │
│                               └─────────────────┘   │
│                               (0.1 BTC = miner fee) │
└─────────────────────────────────────────────────────┘
```

The 0.1 BTC difference between inputs and outputs is the **miner fee** —
it is not an explicit output. Miners collect it as a reward for including
your transaction in a block.

---

## Transaction Structure

A full transaction is made of these parts:

```
Transaction
├── version
├── inputs[]
│   ├── prev_txid
│   ├── prev_vout
│   ├── scriptSig (signature + public key)
│   └── sequence
├── outputs[]
│   ├── value
│   └── scriptPubKey (locking script / address)
└── locktime
```

---

## Fields in Detail

### `version`

A small number that tells the network which rules apply to this transaction.

- **Version 1** — the original format, basic transactions
- **Version 2** — introduced relative timelocks (you can say "this input cannot
  be spent until N blocks after the tx it references was mined")

For your blockchain, always use `1` to start.

---

### `inputs`

Every input points to a previous output that you want to spend.
You cannot invent coins out of thin air — you must reference existing UTXOs.

An input has:

#### `prev_txid: [u8; 32]`
The ID (double SHA-256 hash) of the transaction that created the output you want to spend.
This is how the network knows *which past transaction* you are referring to.

#### `prev_vout: u32`

A single transaction can produce **multiple outputs** at once — one for the recipient,
one for your change, possibly more. They are stored in an ordered list inside the
transaction. `vout` is the **index** (0, 1, 2, ...) of the specific slot you want
to reference. Think of it as pointing at a row in a table.

```
  Transaction ID: "abc123"
  ┌───────┬────────────────────────────────────────────┐
  │ Index │ Output                                     │
  ├───────┼────────────────────────────────────────────┤
  │   0   │  3.0 BTC  → locked to Alice's address      │  ← vout = 0
  │   1   │  6.9 BTC  → locked to Bob's address        │  ← vout = 1
  └───────┴────────────────────────────────────────────┘
           (0.1 BTC difference = miner fee, not an output)
```

Later, if Alice wants to spend her coins, her new transaction must reference
exactly which output it is consuming:

```
  Alice's new transaction
  ┌─────────────────────────────────────────────────────┐
  │  INPUT                                              │
  │  ┌────────────────────────────────────────────────┐ │
  │  │  prev_txid: "abc123"  ◀── which transaction    │ │
  │  │  prev_vout: 0         ◀── which output (row 0) │ │
  │  └────────────────────────────────────────────────┘ │
  │            │                                        │
  │            └──▶ consumes the 3.0 BTC from row 0    │
  └─────────────────────────────────────────────────────┘
```

Without `prev_vout`, the node would not know which of the two outputs Alice is
spending. The pair `(prev_txid, prev_vout)` is a globally unique pointer to
exactly one output in the entire history of the blockchain.

#### `scriptSig` — The Unlocking Script

`scriptSig` is the **proof** you attach to your input that says:
*"I am allowed to spend this output."*

To understand it properly, you first need to understand what `scriptPubKey` is
(covered in the Outputs section below) — but here is the short version:

Every output in Bitcoin is **locked** with a small program called `scriptPubKey`.
To spend that output, you must provide a matching `scriptSig` that **satisfies**
the lock — like a key fitting a padlock.

```
  OUTPUT (created when someone sent you coins)
  ┌──────────────────────────────────────────┐
  │  value: 3 BTC                            │
  │  scriptPubKey: "LOCKED — only the owner  │
  │                of address XYZ can open"  │  ← the padlock
  └──────────────────────────────────────────┘

  INPUT (when you spend those coins later)
  ┌──────────────────────────────────────────┐
  │  prev_txid: abc123                       │
  │  prev_vout: 0                            │
  │  scriptSig: <your signature>             │  ← the key
  │             <your public key>            │
  └──────────────────────────────────────────┘
```

When a node validates your transaction, it runs both scripts together and checks
if the result says "valid". If yes — you are allowed to spend. If no — rejected.

The `scriptSig` for the most common type of transaction (P2PKH) contains exactly
two things:
1. Your **digital signature** (proves you authorized this specific transaction)
2. Your **public key** (proves your key matches the locked address)

More detail on how the locking/unlocking actually works is in the
Scripts section below.

#### `sequence: u32`
Originally meant for replacing unconfirmed transactions.
Today mostly used for relative timelocks (version 2).
Default value: `0xFFFFFFFF` (disabled / no locktime).

---

### `outputs`

Each output defines where the money goes and who can spend it next.

#### `value: u64`
The amount being sent, measured in **satoshis**.

```
1 BTC  = 100,000,000 satoshis
0.1 BTC =  10,000,000 satoshis
```

Using `u64` (unsigned 64-bit integer) is intentional — it avoids floating point
rounding errors when dealing with money. All Bitcoin math is done in integers.

#### `scriptPubKey` — The Locking Script

`scriptPubKey` is a **small program embedded in every output** that defines the
conditions someone must meet to spend that output in the future.

Think of it as a combination lock on a safe. You put money in the safe and set
a combination. Only someone who knows the combination can open it.

```
  When Bob sends Alice 3 BTC:

  Bob's wallet says:
  "Create an output worth 3 BTC, locked with Alice's address"

  The output gets this scriptPubKey burned into it:
  ┌────────────────────────────────────────────────────────┐
  │  scriptPubKey:                                         │
  │  "The spender must prove they own address 1Alice..."   │
  │   ↑ this is stored permanently in the blockchain       │
  └────────────────────────────────────────────────────────┘

  Later, ONLY Alice can create a valid scriptSig to open it.
  Anyone else's key will not fit.
```

**Scripts are programs, not just data.** Bitcoin has a small scripting language
called **Script** (stack-based, like an old calculator). `scriptPubKey` is a
short program written in this language.

The most common type is **P2PKH (Pay To Public Key Hash)**. Here is what it
actually looks like as Script opcodes:

```
  scriptPubKey (P2PKH):
  ┌────────────────────────────────────────────────────────────────┐
  │  OP_DUP  OP_HASH160  <Alice's pubkey hash>  OP_EQUALVERIFY  OP_CHECKSIG  │
  └────────────────────────────────────────────────────────────────┘

  Translation in plain English:
  ┌───────────────┬────────────────────────────────────────────────┐
  │ OP_DUP        │ Duplicate the top item on the stack            │
  │ OP_HASH160    │ Hash it (SHA-256 then RIPEMD-160)              │
  │ <pubkey hash> │ Push Alice's address onto the stack            │
  │ OP_EQUALVERIFY│ Check the two hashes match — fail if not       │
  │ OP_CHECKSIG   │ Verify the signature is valid — fail if not    │
  └───────────────┴────────────────────────────────────────────────┘
```

You do not need to memorise the opcodes. The key point is:
**scriptPubKey says "prove you own this address".**

---

---

### Scripts — How Locking and Unlocking Actually Works

This is the mechanism behind `scriptPubKey` and `scriptSig`. Understanding it
removes all the mystery around how Bitcoin enforces ownership.

#### The Lock and Key Relationship

Every output has a lock (`scriptPubKey`). To spend it, your input must provide
the matching key (`scriptSig`). The node combines both scripts and runs them:

```
  ┌──────────────────────────────────────────────────────────────┐
  │                    VALIDATION                                │
  │                                                              │
  │   scriptSig          +        scriptPubKey                   │
  │   (from your input)           (from the output you're        │
  │                                spending)                     │
  │                                                              │
  │   <signature>                 OP_DUP                         │
  │   <public_key>                OP_HASH160                     │
  │                               <Alice's pubkey hash>          │
  │                               OP_EQUALVERIFY                 │
  │                               OP_CHECKSIG                    │
  │                                                              │
  │   Combined and run together ──▶  TRUE or FALSE               │
  │                                                              │
  │   TRUE  = you are allowed to spend this output               │
  │   FALSE = transaction rejected                               │
  └──────────────────────────────────────────────────────────────┘
```

#### The Stack — How Script Executes

Script is a **stack-based language**. A stack is like a pile of plates —
you can only put something on top (push) or take from the top (pop).

There are no variables. No loops. Just a sequence of instructions that
push data onto the stack or pop data off and do something with it.

Here is the step-by-step execution of a P2PKH unlock:

```
  Starting state — stack is empty:
  │     │
  └─────┘

  ── scriptSig runs first ──

  Push <signature>:
  │ SIG │
  └─────┘

  Push <public_key>:
  │ PUB │  ← top
  │ SIG │
  └─────┘

  ── scriptPubKey runs second ──

  OP_DUP — duplicate the top item:
  │ PUB │  ← copy
  │ PUB │
  │ SIG │
  └─────┘

  OP_HASH160 — hash the top item (SHA-256 then RIPEMD-160):
  │HASH │  ← hash of PUB
  │ PUB │
  │ SIG │
  └─────┘

  Push <Alice's pubkey hash from the scriptPubKey>:
  │ALICE│  ← Alice's stored address hash
  │HASH │
  │ PUB │
  │ SIG │
  └─────┘

  OP_EQUALVERIFY — pop top two, check they are equal:
  If ALICE == HASH  →  continue (both popped)
  If ALICE != HASH  →  FAIL immediately (wrong address)

  │ PUB │
  │ SIG │
  └─────┘

  OP_CHECKSIG — pop public key and signature, verify:
  The node checks: was this transaction signed with the
  private key that matches PUB?

  If YES  →  push TRUE onto stack  →  VALID ✓
  If NO   →  push FALSE            →  INVALID ✗

  │TRUE │
  └─────┘
```

#### Why This Design Is Brilliant

The output never stores your private key or even your public key directly —
only the **hash** of your public key. This means:

1. **Privacy** — just from looking at the output, nobody knows your public key
2. **Security** — your public key is only revealed at the moment you spend,
   at which point the coins are already moving away anyway
3. **No central authority** — every node runs this script independently and
   reaches the same yes/no conclusion

```
  SENDING coins to Alice:
  Bob knows only Alice's address  (hash of her public key)
         ↓
  scriptPubKey stores the hash
         ↓
  Alice's actual public key is NEVER seen by Bob

  SPENDING coins as Alice:
  Alice reveals her public key in scriptSig
         ↓
  Node hashes it and compares to stored hash  →  match ✓
  Node verifies her signature                 →  valid ✓
         ↓
  Transaction accepted
```

---

### `locktime: u32`

Delays when a transaction becomes valid.

| Value | Meaning |
|---|---|
| `0` | Valid immediately, can be mined in any block |
| `1` to `499_999_999` | Valid only after this **block height** |
| `500_000_000` and above | Valid only after this **Unix timestamp** |

Example: if `locktime = 850000`, the transaction cannot be included in any block
before block number 850,000.

Use case: "I am paying you now, but you cannot touch the money until next year."

---

## Ownership and Signatures — How You Prove the Coins Are Yours

This is the most important security concept in Bitcoin.

### Keys

Every user has a **key pair** derived through a one-way chain:

```
  ┌─────────────────────────────────────────────────────────────────────┐
  │                                                                     │
  │   Random 256-bit number                                             │
  │   ┌──────────────────────┐                                          │
  │   │    PRIVATE KEY       │  ← never share, never loses, master key  │
  │   │  5Kb8kLf9zgWQn...    │                                          │
  │   └──────────┬───────────┘                                          │
  │              │                                                      │
  │              │  secp256k1 elliptic curve multiplication             │
  │              │  (one-way: forward is easy, backward is impossible)  │
  │              ▼                                                      │
  │   ┌──────────────────────┐                                          │
  │   │    PUBLIC KEY        │  ← safe to share with anyone             │
  │   │  04a1b2c3d4e5f6...   │                                          │
  │   └──────────┬───────────┘                                          │
  │              │                                                      │
  │              │  SHA-256, then RIPEMD-160, then Base58Check encode   │
  │              │  (one-way: you cannot get public key from address)   │
  │              ▼                                                      │
  │   ┌──────────────────────┐                                          │
  │   │    ADDRESS           │  ← this is what you give people to pay you│
  │   │  1A1zP1eP5QGefi...   │                                          │
  │   └──────────────────────┘                                          │
  │                                                                     │
  └─────────────────────────────────────────────────────────────────────┘
```

The chain is **one-way at every step**:
- You can go from private key → public key → address
- You cannot go backwards — knowing someone's address tells you nothing about
  their public key, and knowing their public key tells you nothing about their
  private key

### Signing

When you want to spend an output, you must prove you own the private key behind
the address — **without revealing the private key itself**.

You do this by creating a **digital signature** over the transaction data:

```
  ┌──────────────────────────────────────────────────────────────┐
  │  SIGNING                                                     │
  │                                                              │
  │  transaction data ──┐                                        │
  │                     ├──▶  sign()  ──▶  SIGNATURE            │
  │  private key     ───┘                  (a blob of bytes)     │
  │                                                              │
  │  The signature is unique to:                                 │
  │    - this exact transaction (change one byte → invalid sig)  │
  │    - this exact private key (no other key can produce it)    │
  └──────────────────────────────────────────────────────────────┘
```

The signature is placed in `scriptSig`. The private key is **never included** —
it stays secret on your machine forever.

### Verification

Any node can verify your signature using only your **public key** (which you do
include in `scriptSig`):

```
  ┌──────────────────────────────────────────────────────────────┐
  │  VERIFICATION (done by every node)                           │
  │                                                              │
  │  signature       ──┐                                         │
  │  public key      ──┼──▶  verify()  ──▶  TRUE / FALSE        │
  │  transaction data──┘                                         │
  │                                                              │
  │  TRUE  means:                                                │
  │    - This transaction was signed by whoever owns             │
  │      the private key matching this public key                │
  │    - The transaction data has not been altered since signing  │
  │                                                              │
  │  FALSE means:                                                │
  │    - Wrong key, or the transaction was tampered with         │
  └──────────────────────────────────────────────────────────────┘
```

The node also checks that the public key you provided **hashes to the address**
stored in the output's `scriptPubKey`. This confirms you are the intended recipient:

```
  hash(your public key)  ==  address in scriptPubKey  ?
        YES ──▶ you are the right person
        NO  ──▶ rejected, you are not the owner
```

This is **ECDSA** (Elliptic Curve Digital Signature Algorithm) on the **secp256k1** curve.

---

## The Full Flow — Step by Step

Let's say Bob wants to send Alice 2 BTC. Bob has one UTXO worth 5 BTC.

```
  STEP 1 — Bob's wallet finds his UTXO
  ┌───────────────────────────────────────────────────────┐
  │  Bob's UTXO (from a previous transaction "xyz789")    │
  │  ┌────────────────────────────────────────────────┐   │
  │  │  txid: xyz789                                  │   │
  │  │  vout: 0                                       │   │
  │  │  value: 5 BTC                                  │   │
  │  │  scriptPubKey: locked to Bob's address         │   │
  │  └────────────────────────────────────────────────┘   │
  └───────────────────────────────────────────────────────┘
```

```
  STEP 2 — Bob builds the transaction
  ┌───────────────────────────────────────────────────────────────┐
  │  NEW TRANSACTION                                              │
  │                                                               │
  │  INPUT                           OUTPUTS                      │
  │  ┌─────────────────────────┐     ┌───────────────────────┐   │
  │  │ prev_txid: xyz789       │────▶│ vout 0                │   │
  │  │ prev_vout: 0            │  │  │ value: 2.0 BTC        │   │
  │  │ scriptSig: (empty yet)  │  │  │ scriptPubKey:         │   │
  │  └─────────────────────────┘  │  │ locked to Alice       │   │
  │                                │  └───────────────────────┘   │
  │                                │  ┌───────────────────────┐   │
  │                                └─▶│ vout 1                │   │
  │                                   │ value: 2.9 BTC        │   │
  │                                   │ scriptPubKey:         │   │
  │                                   │ locked to Bob         │   │
  │                                   └───────────────────────┘   │
  │                                   (0.1 BTC = miner fee)       │
  └───────────────────────────────────────────────────────────────┘
```

```
  STEP 3 — Bob signs
  ┌───────────────────────────────────────────────────────┐
  │  transaction bytes ──┐                                │
  │                      ├──▶ sign() ──▶ <signature>      │
  │  Bob's private key ──┘                                │
  │                                                       │
  │  scriptSig is now filled in:                          │
  │  [ <signature>  <Bob's public key> ]                  │
  └───────────────────────────────────────────────────────┘
```

```
  STEP 4 — Broadcast and validation by every node
  ┌───────────────────────────────────────────────────────┐
  │  Does xyz789 output 0 exist and is it unspent?   ✓   │
  │  Does Bob's public key hash == address on vout 0? ✓   │
  │  Is the signature valid?                          ✓   │
  │  Is total output (4.9) <= total input (5.0)?     ✓   │
  └───────────────────────────────────────────────────────┘
```

```
  STEP 5 — Miner includes it in a block. UTXO set updates:

  BEFORE:                           AFTER:
  ┌──────────────────────────┐      ┌──────────────────────────┐
  │ UTXO SET                 │      │ UTXO SET                 │
  │                          │      │                          │
  │ xyz789:0  5 BTC  Bob  ✓  │ ───▶ │ xyz789:0  SPENT  ✗      │
  │ ...other UTXOs...        │      │ newtx:0   2 BTC  Alice ✓ │
  └──────────────────────────┘      │ newtx:1   2.9 BTC Bob  ✓ │
                                    │ ...other UTXOs...        │
                                    └──────────────────────────┘

  Old UTXO consumed. Two new UTXOs created. Ownership transferred.
```

---

## The Coinbase Transaction — Where New Coins Come From

Every block has one special transaction called the **coinbase transaction**.
It has no inputs (no prev_txid). It creates new BTC out of nothing.

This is the **block reward** — the miner's payment for doing the work of mining.
Bitcoin started at 50 BTC per block and halves every 210,000 blocks.

This is the only way new Bitcoin ever enters circulation.

---

## Summary

```
Coins don't exist as balances.
They exist as unspent outputs (UTXOs).

To send money:
  - Pick UTXOs as inputs (must be yours)
  - Sign to prove ownership
  - Define outputs (recipient + change)
  - Difference between in and out = miner fee

To receive money:
  - Give someone your address (hash of your public key)
  - They lock an output to it
  - You can spend it later with your private key
```
