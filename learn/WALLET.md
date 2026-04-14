# How a Person Interacts With the Blockchain — A Beginner's Guide

---

## Abbreviations

| Abbreviation | Stands For |
|---|---|
| wallet | Software (or hardware) that manages your private keys and talks to the blockchain |
| private key | A secret 256-bit number that proves ownership of coins |
| public key | A number derived from your private key — safe to share |
| address | A hashed, encoded version of your public key — what you give people to receive BTC |
| seed phrase | 12 or 24 words that encode your master key — the ultimate backup |
| HD wallet | Hierarchical Deterministic wallet — generates all keys from one seed |
| UTXO | Unspent Transaction Output — the actual "coins" on the blockchain |
| balance | The sum of all UTXOs locked to your addresses, calculated by your wallet |
| broadcast | Sending your signed transaction to the network |
| confirmation | Each new block built on top of the block containing your transaction |
| hot wallet | A wallet whose private key is on an internet-connected device |
| cold wallet | A wallet whose private key is stored offline (hardware device) |
| custodial | A service (like an exchange) that holds your private key for you |
| non-custodial | A wallet where only YOU hold the private key |

---

## What a Wallet Actually Is — and Is NOT

This is the most common misunderstanding in all of Bitcoin.

**A wallet does NOT store coins.**

Coins do not exist as files on your phone. There is nothing to download, nothing
to back up, nothing that could be "in" your phone and lost if it breaks.

**A wallet stores private keys** — the secret numbers that prove you are allowed
to move specific coins on the blockchain.

The coins themselves (UTXOs) live on the blockchain, which is replicated across
thousands of nodes worldwide. Your wallet is just the tool that holds the key to
access them.

```
  WRONG mental model:
  ┌─────────────────────┐
  │   YOUR PHONE        │
  │  ┌───────────────┐  │
  │  │  0.5 BTC      │  │  ← coins are NOT here
  │  └───────────────┘  │
  └─────────────────────┘

  CORRECT mental model:
  ┌─────────────────────┐        ┌──────────────────────────────┐
  │   YOUR PHONE        │        │   BLOCKCHAIN (everywhere)    │
  │  ┌───────────────┐  │        │                              │
  │  │  PRIVATE KEY  │──┼───────▶│  UTXO: 0.5 BTC              │
  │  │  (the key)    │  │        │  locked to your address ✓    │
  │  └───────────────┘  │        │                              │
  └─────────────────────┘        └──────────────────────────────┘
       keychain                          the safe
```

Your wallet is a **keychain**. The blockchain is the **safe**. The safe exists
everywhere simultaneously. If you lose the keychain but have a copy of the key,
you can open the safe from any device, anywhere in the world.

---

## What You Actually Own

When people say "I have 0.5 BTC", what they really own is this:

```
  YOU OWN:  a private key
                 ↓
  THAT KEY CONTROLS:  one or more addresses
                 ↓
  THOSE ADDRESSES HAVE:  UTXOs locked to them on the blockchain
                 ↓
  YOUR WALLET ADDS THEM UP:  shows you "0.5 BTC"
```

The blockchain has no concept of you as a person. It does not know your name,
your country, or your email. It only knows **addresses** — anonymous strings of
letters and numbers. Your wallet is the bridge between you (a human) and your
addresses (anonymous identifiers on a public ledger).

```
  BLOCKCHAIN perspective:
  ┌──────────────────────────────────────────────────────────┐
  │  Address 1A1zP...  has  0.2 BTC                          │
  │  Address 1BvBMS...  has  0.3 BTC                         │
  │  Address 1GkQm...  has  0.05 BTC                         │
  │  ... millions more addresses ...                         │
  └──────────────────────────────────────────────────────────┘
  (the blockchain has no idea these three belong to the same person)

  YOUR WALLET perspective:
  ┌──────────────────────────────────────────────────────────┐
  │  I control these addresses:                              │
  │    1A1zP...   →  0.20 BTC  ✓                             │
  │    1BvBMSE... →  0.30 BTC  ✓                             │
  │    1GkQmT...  →  0.05 BTC  ✓                             │
  │                  ──────────                              │
  │  YOUR BALANCE:   0.55 BTC                                │
  └──────────────────────────────────────────────────────────┘
```

---

## Addresses — What They Are and Where They Come From

An **address** is what you give someone when you want to receive Bitcoin.
It looks like this:

```
  1A1zP1eP5QGefi2DMPTfTL5SLmv7Divf Na
```

It is derived from your public key through a chain of hashing:

```
  Private Key
       │
       │  elliptic curve math (secp256k1)
       ▼
  Public Key
       │
       │  SHA-256 → RIPEMD-160 → Base58Check encoding
       ▼
  Address  (what you share with the world)
```

One private key can generate **infinite addresses**. Modern wallets (HD wallets)
do this automatically — every time you hit "Receive", they hand you a fresh address:

```
  ONE PRIVATE KEY (or seed)
         │
         ├──▶  Address 1:  1A1zP...   (used to receive payment from Alice)
         ├──▶  Address 2:  1BvBM...   (used to receive payment from Bob)
         ├──▶  Address 3:  1GkQm...   (used as change output)
         ├──▶  Address 4:  1Feex...   (ready for next receive)
         └──▶  ... infinite more ...
```

All of these addresses are **controlled by the same private key**. Your wallet
tracks all of them and sums up their UTXOs into your balance.

**Why use a new address each time?**
Privacy. If you reuse the same address, anyone can look at the blockchain and
see every transaction that address has ever been involved in. Fresh addresses
make it much harder to track your activity.

---

## The Seed Phrase — Your Master Backup

When you set up a wallet for the first time, it shows you 12 or 24 random words:

```
  witch collapse practice feed shame open
  despair creek road again ice least
```

This is your **seed phrase**. It is the most important thing in your Bitcoin life.

```
  SEED PHRASE (12 or 24 words)
         │
         │  BIP-39 encoding
         ▼
  MASTER PRIVATE KEY  (512-bit number)
         │
         │  BIP-32 derivation
         ▼
  ┌──────────────────────────────────────────┐
  │  Private Key 1  →  Address 1, 2, 3...   │
  │  Private Key 2  →  Address 4, 5, 6...   │
  │  Private Key 3  →  Address 7, 8, 9...   │
  │  ... all derived deterministically ...  │
  └──────────────────────────────────────────┘
```

From that one seed, your wallet can regenerate **every single private key and
address it has ever created** — in the exact same order, every time.

This means:

```
  You lose your phone
         ↓
  Install any wallet app on a new phone
         ↓
  Enter your 12/24 seed words
         ↓
  Wallet regenerates all your keys and addresses
         ↓
  Scans the blockchain for UTXOs on those addresses
         ↓
  Full balance restored — nothing was lost
```

But also:

```
  Someone finds your written-down seed phrase
         ↓
  They enter it in their wallet
         ↓
  They have complete control of all your coins
         ↓
  You cannot stop them — there is no password reset, no support line
```

**The seed phrase IS your wallet.** Not the app. Not the device. The words.
Write them down on paper. Store in a safe place. Never type them into any website.

---

## Hot Wallet vs Cold Wallet vs Custodial

```
  ┌─────────────────┬──────────────────────────┬──────────┬────────────┐
  │ Type            │ Who holds the key         │ Security │ Convenience│
  ├─────────────────┼──────────────────────────┼──────────┼────────────┤
  │ Hot wallet      │ You — on internet device  │ Medium   │ High       │
  │ (phone/desktop) │ (phone, laptop)           │          │            │
  ├─────────────────┼──────────────────────────┼──────────┼────────────┤
  │ Cold wallet     │ You — on offline device   │ High     │ Medium     │
  │ (Ledger, Trezor)│ (hardware wallet)         │          │            │
  ├─────────────────┼──────────────────────────┼──────────┼────────────┤
  │ Custodial       │ A company holds it for you│ Varies   │ Very high  │
  │ (Coinbase etc.) │ You trust them            │          │            │
  └─────────────────┴──────────────────────────┴──────────┴────────────┘
```

**Custodial warning:**

When you buy Bitcoin on an exchange and leave it there, you do not own Bitcoin.
You own an IOU from that company. They hold the actual private key.

If the exchange is hacked, goes bankrupt, or freezes your account — your coins
can be gone. This has happened many times (Mt. Gox 2014, FTX 2022).

```
  "Not your keys, not your coins."
```

---

## What Happens When You Hit "Receive"

```
  You tap "Receive" in your wallet app
         │
         ▼
  Wallet generates (or shows) one of your addresses
  ┌──────────────────────────────────────┐
  │  [ QR CODE ]                         │
  │  1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa │
  └──────────────────────────────────────┘
         │
         ▼
  You share this address with the sender
  (nothing has happened on the blockchain yet —
   you have just handed them a mailbox address)
         │
         ▼
  Sender broadcasts a transaction to that address
         │
         ▼
  Your wallet detects it in the mempool
  → shows "0.5 BTC (pending)"
         │
         ▼
  Transaction gets confirmed in a block
  → A new UTXO locked to your address now exists on-chain
  → wallet shows "0.5 BTC (1 confirmation)"
         │
         ▼
  After 6 confirmations → fully settled, safe to spend
```

You did not need to be online for any of this after sharing the address.
The UTXO simply appeared on-chain, locked to your address, waiting for you.

---

## What Happens When You Hit "Send" — Every Single Step

This is the full picture, from your finger on the screen to the recipient
seeing confirmed funds.

```
  STEP 1 — You enter the details
  ┌────────────────────────────────────────┐
  │  Send to:  1Alice...                  │
  │  Amount:   0.3 BTC                    │
  │  Fee:      [estimated by wallet]      │
  │                    [ SEND ]           │
  └────────────────────────────────────────┘
```

```
  STEP 2 — Wallet picks your UTXOs (coin selection)
  ┌────────────────────────────────────────────────────────┐
  │  Your UTXOs:                                           │
  │    UTXO A:  0.5 BTC  (from tx abc123, vout 0)  ← pick │
  │    UTXO B:  0.2 BTC  (from tx def456, vout 1)         │
  │                                                        │
  │  0.5 BTC is enough to cover 0.3 BTC + fee             │
  └────────────────────────────────────────────────────────┘
```

```
  STEP 3 — Wallet builds the transaction
  ┌────────────────────────────────────────────────────────┐
  │  INPUT                                                 │
  │  └── UTXO A: 0.5 BTC (abc123, vout 0)                 │
  │                                                        │
  │  OUTPUTS                                               │
  │  ├── vout 0: 0.3 BTC  → Alice's address  (payment)    │
  │  └── vout 1: 0.19 BTC → YOUR new address (change)     │
  │                                                        │
  │  fee: 0.01 BTC (0.5 - 0.3 - 0.19 = 0.01)             │
  └────────────────────────────────────────────────────────┘
```

```
  STEP 4 — Wallet signs (you enter your PIN to unlock the key)
  ┌────────────────────────────────────────────────────────┐
  │  transaction data ──┐                                  │
  │                     ├──▶ sign() ──▶ <signature>        │
  │  your private key ──┘                                  │
  │                                                        │
  │  scriptSig now contains: <signature> + <public key>    │
  │  Transaction is sealed — nothing can change now        │
  └────────────────────────────────────────────────────────┘
```

```
  STEP 5 — Wallet broadcasts to the network
  ┌────────────────────────────────────────────────────────┐
  │  Raw transaction bytes sent to a connected node        │
  │                                                        │
  │  That node validates and relays to its peers           │
  │  Those peers relay to their peers                      │
  │  Within ~2 seconds: thousands of nodes have it         │
  │                                                        │
  │  Wallet shows: "Pending..."                            │
  └────────────────────────────────────────────────────────┘
```

```
  STEP 6 — Mempool waiting
  ┌────────────────────────────────────────────────────────┐
  │  Transaction floats in the mempool                     │
  │  UTXO A is now "reserved" — cannot be spent again      │
  │  Duration: seconds to hours depending on fee           │
  └────────────────────────────────────────────────────────┘
```

```
  STEP 7 — A miner includes it in a block
  ┌────────────────────────────────────────────────────────┐
  │  BLOCK #850,100                                        │
  │  ├── coinbase tx (miner reward)                        │
  │  ├── your transaction  ← HERE                          │
  │  ├── 2,300 other transactions                          │
  │  └── ...                                               │
  │                                                        │
  │  Wallet shows: "1 confirmation"                        │
  └────────────────────────────────────────────────────────┘
```

```
  STEP 8 — More blocks confirm it
  ┌────────────────────────────────────────────────────────┐
  │  Block 850,101 built on top  →  2 confirmations        │
  │  Block 850,102 built on top  →  3 confirmations        │
  │  ...                                                   │
  │  Block 850,105 built on top  →  6 confirmations        │
  │                                                        │
  │  Wallet shows: "Confirmed ✓"                           │
  │  Alice's wallet shows: "0.3 BTC received ✓"            │
  └────────────────────────────────────────────────────────┘
```

---

## What "Balance" Really Is

There is **no balance field** anywhere in the Bitcoin blockchain.
Nobody tracks your running total. The blockchain only records UTXOs.

Your wallet calculates your balance by doing this:

```
  ┌──────────────────────────────────────────────────────────┐
  │  WALLET BALANCE CALCULATION                              │
  │                                                          │
  │  1. Wallet knows all your addresses                      │
  │     [1A1zP..., 1BvBM..., 1GkQm..., ...]                 │
  │                                                          │
  │  2. Wallet queries the blockchain for UTXOs              │
  │     locked to those addresses                            │
  │                                                          │
  │     Address 1A1zP...  →  UTXO: 0.10 BTC                 │
  │     Address 1BvBM...  →  UTXO: 0.30 BTC                 │
  │     Address 1GkQm...  →  UTXO: 0.15 BTC                 │
  │     Address 1FeexV... →  nothing                         │
  │                                                          │
  │  3. Wallet adds them up                                  │
  │     0.10 + 0.30 + 0.15 = 0.55 BTC                       │
  │                                                          │
  │  4. Wallet displays:  "Balance: 0.55 BTC"                │
  └──────────────────────────────────────────────────────────┘
```

Every time you open your wallet app, it goes through this process.
The number you see is just a convenient summary of raw blockchain data.

---

## What Happens to Your Balance After Sending

Let's say you have one UTXO of 1.0 BTC and you send 0.3 BTC.

```
  BEFORE SENDING:
  ┌──────────────────────────────────────────┐
  │  Your UTXOs:                             │
  │  ┌──────────────────────────────────┐    │
  │  │  UTXO A: 1.0 BTC                 │    │
  │  │  address: 1A1zP...               │    │
  │  └──────────────────────────────────┘    │
  │                                          │
  │  Displayed balance:  1.0 BTC             │
  └──────────────────────────────────────────┘

  WHILE PENDING (transaction in mempool):
  ┌──────────────────────────────────────────┐
  │  UTXO A is "reserved" — outgoing         │
  │  Displayed balance:  0.0 BTC available   │
  │                      (or 1.0 BTC - 0.3   │
  │                       shown as pending)  │
  └──────────────────────────────────────────┘

  AFTER CONFIRMED (transaction in a block):
  ┌──────────────────────────────────────────┐
  │  Your UTXOs:                             │
  │  ┌──────────────────────────────────┐    │
  │  │  UTXO A: SPENT — gone forever    │    │
  │  └──────────────────────────────────┘    │
  │  ┌──────────────────────────────────┐    │
  │  │  UTXO B: 0.69 BTC  ← your change│    │
  │  │  address: 1NewAddr... (fresh one)│    │
  │  └──────────────────────────────────┘    │
  │                                          │
  │  Displayed balance:  0.69 BTC            │
  │  (Alice now has 0.3 BTC on her side)     │
  │  (0.01 BTC went to the miner as fee)     │
  └──────────────────────────────────────────┘
```

The original UTXO is **destroyed**. Two new UTXOs were created simultaneously
when the block was confirmed — one for Alice, one for your change.
They did not exist before that block.

---

## What an Address Is at the Backend Level

This is the part most people never fully understand. Where does an address
actually *live* inside the blockchain?

It lives inside the **scriptPubKey** of a transaction output. That is the only
place it exists on the blockchain. Nothing else, nowhere else.

When Bob sends money to Alice, his wallet writes this into the output:

```
  TRANSACTION OUTPUT (stored permanently in a block):
  ┌──────────────────────────────────────────────────────────┐
  │  value: 30,000,000 satoshis                              │
  │                                                          │
  │  scriptPubKey:  OP_DUP                                   │
  │                 OP_HASH160                               │
  │                 <Alice's pubkey hash>  ◀── ADDRESS HERE  │
  │                 OP_EQUALVERIFY                           │
  │                 OP_CHECKSIG                              │
  └──────────────────────────────────────────────────────────┘
```

Alice's address **is** that `<pubkey hash>` — 20 bytes embedded directly into
the output script, sealed into the blockchain forever when the block is confirmed.
There is no separate address database. No registry. No lookup table.
The address is just data sitting inside a transaction output.

### Why it works without a central registry

There is no server that maps "Alice = this address". The address is
**self-proving** through math:

```
  Alice's private key
         │
         │  secp256k1 elliptic curve math
         ▼
  Alice's public key
         │
         │  SHA-256 → RIPEMD-160
         ▼
  Alice's pubkey hash  ←── THIS IS HER ADDRESS
         │
         │  embedded into scriptPubKey of every output sent to her
         ▼
  Stored in the blockchain inside transaction outputs
```

When Alice later wants to spend that UTXO, she provides her public key
in `scriptSig`. Every node independently verifies it on the spot:

```
  Node receives Alice's spending transaction
         │
         ▼
  Node hashes the public key Alice provided:
  hash(Alice's public key) = ?

         │
         ├── matches pubkey hash in scriptPubKey? ✓
         │         ↓
         │   Is her signature valid? ✓
         │         ↓
         │   TRANSACTION ACCEPTED
         │
         └── does not match?
                   ↓
             REJECTED — wrong person

  Nobody registered Alice anywhere.
  The math proves ownership. That is the entire system.
```

### One sentence

> An address is a hash of a public key, embedded directly into a transaction
> output as `scriptPubKey` — that is the only place it exists on the blockchain.

---

## What the Wallet Does NOT Do

To close the loop — here is a list of things your wallet app does NOT do,
that many people assume it does:

```
  ✗  The wallet does NOT store your coins
     → coins are UTXOs on the blockchain, not files on your device

  ✗  The wallet does NOT talk to a "Bitcoin company server"
     → it connects directly to nodes on the P2P network
        (or to your own node, or a trusted node)

  ✗  The wallet does NOT need permission from anyone to send
     → no account, no login, no approval required

  ✗  The wallet company CANNOT freeze your funds
     → only the person with the private key can move coins
        (exception: custodial wallets where they hold the key)

  ✗  If the wallet app shuts down, your coins are NOT lost
     → install any other wallet, enter your seed phrase, done

  ✗  The wallet does NOT create transactions automatically
     → every transaction is explicitly initiated by you
```

The wallet app is **replaceable**. Your seed phrase is **not**.

---

## Summary

```
A wallet = a keychain that holds private keys.
           Not a bank. Not a storage box. Just keys.

What you own:
  private key → controls addresses → addresses have UTXOs on-chain
  "balance" = your wallet summing up those UTXOs

Seed phrase = master backup of all your keys.
              Lose it = lose everything.
              Someone else gets it = they have everything.

Receiving:
  Share your address → sender broadcasts a tx to it
  → UTXO appears on-chain locked to your address → balance goes up

Sending:
  You enter amount + recipient
  → wallet picks UTXOs → builds tx (payment + change outputs)
  → signs with private key → broadcasts to network
  → mempool → miner includes in block → confirmed

Balance:
  Not stored anywhere. Your wallet scans all your addresses
  on the blockchain and adds up their UTXOs.

Hot wallet   = convenient, key on internet device
Cold wallet  = secure, key on offline device
Custodial    = THEY hold the key, you trust them ("not your keys, not your coins")
```
