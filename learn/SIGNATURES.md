# Digital Signatures — How They Are Formed and How They Work

---

## Abbreviations

| Abbreviation | Stands For |
|---|---|
| ECDSA | Elliptic Curve Digital Signature Algorithm |
| secp256k1 | The specific elliptic curve Bitcoin uses |
| private key | A secret 256-bit number — the master proof of ownership |
| public key | A point on the elliptic curve derived from the private key |
| signature | A pair of numbers (r, s) produced by signing with a private key |
| scriptSig | The unlocking script placed in an input — contains signature + public key |
| scriptPubKey | The locking script placed in an output — contains the address (pubkey hash) |
| hash | A fixed-size fingerprint of data produced by SHA-256 |
| DER | Distinguished Encoding Rules — the format used to encode a signature as bytes |

---

## What a Signature Actually Is

A digital signature is **not** a scan of your handwritten name.
It is a **pair of two numbers**: `(r, s)`.

```
  signature = (sig_r, sig_s)

  sig_r  →  the x coordinate of a random point on the elliptic curve
  sig_s  →  a number calculated from sig_r, your private_key, and tx_hash
```

Both `sig_r` and `sig_s` are 256-bit integers. Together they are encoded into
~71 bytes (DER format) and placed into `scriptSig`.

That pair of numbers is mathematically linked to:
1. Your **private_key** (proves only you could have created it)
2. The **exact transaction data** being signed via **tx_hash** (proves it hasn't been tampered with)

If either changes — different private_key or different transaction — the signature
is completely different and fails verification.

---

## How the Private Key Is Formed — Where It Comes From

The private key is not computed from anything. It is not derived. It is not assigned.

It is simply a **random number** — generated fresh on your device the first time
you create a wallet.

### Baby explanation

Imagine you have a bag with 2²⁵⁶ numbered balls inside.
You close your eyes, reach in, and pull one out.
That number on the ball is your private key.
Nobody tells you which one to pick. Nobody knows which one you picked.
You just randomly grab one.

```
  2²⁵⁶ possible private keys:
  ┌────────────────────────────────────────────────────────────┐
  │  115,792,089,237,316,195,423,570,985,008,687,907,853,      │
  │  269,984,665,640,564,039,457,584,007,913,129,639,936       │
  │                                                            │
  │  That is roughly the number of atoms in the observable     │
  │  universe — squared.                                       │
  │                                                            │
  │  You randomly pick 1 of these.                             │
  └────────────────────────────────────────────────────────────┘

  Chance two people generate the same key:
  essentially zero — less likely than picking the same specific
  atom twice from the entire observable universe
```

### Where the randomness comes from

Your operating system collects **entropy** — unpredictable physical data:

```
  ┌─────────────────────────────────────────────────────────┐
  │  ENTROPY SOURCES (unpredictable real-world noise)       │
  │                                                         │
  │  CPU timing variations       ──┐                        │
  │  Mouse movement coordinates  ──┤                        │
  │  Keyboard timing             ──┼──▶  OS entropy pool    │
  │  Network packet timing       ──┤         │              │
  │  Hardware random generators  ──┘         │              │
  │                                          │ mix + hash   │
  │                                          ▼              │
  │                                   32 random bytes       │
  │                                          │              │
  │                                          ▼              │
  │                                   PRIVATE KEY           │
  └─────────────────────────────────────────────────────────┘
```

The private key must be in the range `[1, n-1]` where `n` is the curve order:

```
  n = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141

  (a specific large prime number, part of the secp256k1 standard)
  (if the random number falls outside this range, generate again)
```

### Where the private key lives

```
  ┌──────────────────────────────────────────────────────────┐
  │  PRIVATE KEY LOCATION                                    │
  │                                                          │
  │  ✓  Your wallet app (encrypted on disk)                  │
  │  ✓  Your hardware wallet (inside a secure chip)          │
  │  ✓  Your paper backup / seed phrase                      │
  │                                                          │
  │  ✗  NEVER in a transaction                               │
  │  ✗  NEVER in a block                                     │
  │  ✗  NEVER transmitted over the network                   │
  │  ✗  NEVER on any server                                  │
  └──────────────────────────────────────────────────────────┘
```

---

## What G Is — The Generator Point

This is the thing that confuses everyone. Let me explain it from scratch.

### Baby explanation first

Imagine a game where everyone plays on the same board.
The board has a special square called the **starting square**.
Everyone agrees on which square it is before the game begins.
It is written in the rulebook.

**G is that starting square.**

In Bitcoin, G is a specific point on the secp256k1 curve that was chosen
when the curve was designed and published. It is not secret. It is not special.
It is just the agreed-upon starting point that everyone uses.

```
  G (the generator point) is hardcoded — its coordinates are:

  x: 79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798
  y: 483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8

  Everyone in the world using Bitcoin uses these exact same numbers.
  They are public. You can look them up. They are in the secp256k1 spec.
```

### What "point multiplication" means

The curve `y² = x³ + 7` is a set of points `(x, y)` that satisfy that equation.
You can do math on these points — specifically, you can **add** two points together
and get a third point on the same curve.

Point doubling (adding a point to itself) has a geometric meaning:

```
  THE CURVE (simplified view):

          │         . ← point P (starting here)
          │        /
          │       / ← tangent line at P
          │      /
  ────────┼────/──────────────────────
          │   /
          │  . ← where tangent hits the curve again
          │  │
          │  . ← reflect over x-axis = result of P + P = 2P
          │
```

Step by step:
```
  1. Start at point G on the curve
  2. Draw the tangent line at G
  3. Find where the line hits the curve again
  4. Reflect that point over the x-axis
  5. That is G + G = 2G

  Do this again starting from 2G → get 3G
  Do this again → 4G
  ...
  Do this private_key times → PUBLIC KEY
```

In practice this uses fast algorithms (not literally looping millions of times),
but conceptually: **multiplying G by your private key is just jumping along the
curve a specific number of times**.

```
  private key = 3

  G  ──add──▶  2G  ──add──▶  3G  =  public key
  (start)                   (end)

  private key = 7

  G → 2G → 3G → 4G → 5G → 6G → 7G  =  public key
```

### Why G is the same for everyone

Because **verification math only works if everyone uses the same starting point**.

When a node verifies your signature, it does math involving G.
If your wallet used a different G than the node, the verification would always fail —
even with a valid signature.

G is like the agreed-upon metre stick. Everyone measures in metres.
If you used centimetres and the verifier used metres, the numbers would not match.

```
  G is not:
    ✗  a secret
    ✗  unique per user
    ✗  stored in transactions or blocks
    ✗  something you choose

  G is:
    ✓  a fixed point hardcoded in the secp256k1 standard
    ✓  the same for every Bitcoin wallet, every node, everywhere
    ✓  publicly known — part of the published specification
    ✓  the shared "ruler" that makes signature verification possible
```

---

## Where Every Key Lives — Complete Location Map

This is the question that clears up all confusion. Here is exactly where
every piece of cryptographic data lives:

```
  ┌──────────────┬────────────────┬──────────────────────┬──────────────┬─────────┐
  │ Data         │ In your wallet?│ In a transaction?    │ In a block?  │ Public? │
  ├──────────────┼────────────────┼──────────────────────┼──────────────┼─────────┤
  │ private key  │ YES — only here│ NEVER                │ NEVER        │ NO      │
  ├──────────────┼────────────────┼──────────────────────┼──────────────┼─────────┤
  │ public key   │ YES (derived)  │ YES — in scriptSig   │ YES (in tx)  │ YES     │
  │              │                │ (INPUT, when spending)│             │         │
  ├──────────────┼────────────────┼──────────────────────┼──────────────┼─────────┤
  │ address      │ YES (derived)  │ YES — in scriptPubKey│ YES (in tx)  │ YES     │
  │ (pubkey hash)│                │ (OUTPUT, when receiving)│           │         │
  ├──────────────┼────────────────┼──────────────────────┼──────────────┼─────────┤
  │ signature    │ NO — created   │ YES — in scriptSig   │ YES (in tx)  │ YES     │
  │ (r, s)       │ and discarded  │ (INPUT, when spending)│             │         │
  ├──────────────┼────────────────┼──────────────────────┼──────────────┼─────────┤
  │ G (generator)│ NO             │ NO                   │ NO           │ YES     │
  │              │                │                      │ (hardcoded   │         │
  │              │                │                      │ in software) │         │
  └──────────────┴────────────────┴──────────────────────┴──────────────┴─────────┘
```

Now the same information as a visual structure diagram:

```
  YOUR DEVICE (never leaves here):
  ┌──────────────────────────────────────────────────────────┐
  │  WALLET                                                  │
  │  ├── private key  (the secret, source of everything)     │
  │  ├── public key   (derived from private key × G)         │
  │  └── addresses    (derived from hash of public key)      │
  └──────────────────────────────────────────────────────────┘

  ON THE BLOCKCHAIN (public, permanent, everyone can see):
  ┌──────────────────────────────────────────────────────────┐
  │  BLOCK                                                   │
  │  └── TRANSACTION                                         │
  │        │                                                 │
  │        ├── INPUT  (you filling this in when SPENDING)    │
  │        │     └── scriptSig                               │
  │        │           ├── signature (r, s)  ← proof you own │
  │        │           └── public key        ← who you are   │
  │        │                                                 │
  │        └── OUTPUT  (sender fills this when SENDING TO YOU│
  │              └── scriptPubKey                            │
  │                    └── address (hash of your public key) │
  │                          ← the lock only your key opens  │
  └──────────────────────────────────────────────────────────┘

  HARDCODED IN SOFTWARE (not stored anywhere, just known):
  ┌──────────────────────────────────────────────────────────┐
  │  G = the generator point                                 │
  │  (same in every wallet app and every node worldwide)     │
  └──────────────────────────────────────────────────────────┘
```

### The key insight

```
  private key   →  stays on your device forever, used only to sign locally
  public key    →  revealed on-chain when you SPEND (in scriptSig)
  address       →  on-chain when someone SENDS TO YOU (in scriptPubKey)
  signature     →  on-chain when you SPEND (in scriptSig, alongside public key)
  G             →  hardcoded in the rules, same everywhere, not stored anywhere
```

---

## Step 1 — The Private Key and Public Key

Everything starts with the private key — a random 256-bit number:

```
  private_key:
  e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35
  (just a number, 32 bytes, looks like hex)
```

From this, Bitcoin derives a **public key** using elliptic curve math.
The curve used is **secp256k1**, defined by the equation:

```
  y² = x³ + 7   (over a finite field)
```

The curve looks roughly like this (in real numbers — Bitcoin uses a finite field
but the shape concept holds):

```
         │        *
         │      *   *
         │    *       *
    ─────┼──────────────────
         │    *       *
         │      *   *
         │        *
```

There is a special point on this curve called the **generator point G**,
hardcoded into the secp256k1 standard.

To get the public key:

```
  public_key = private_key × generator

  (this is "point multiplication" on the curve — adding G to itself
   private_key times. Extremely fast forward, impossible to reverse)
```

```
  private key (a number)
         │
         │  × G  (elliptic curve point multiplication)
         │
         ▼
  public_key (a point on the curve — two coordinates)
  = (x_coordinate: a3b1c2..., y_coordinate: f7e8d9...)
```

The public key is a **point** — two 256-bit numbers (x, y). It is usually stored
as 33 bytes (compressed format, just the x coordinate + a parity bit).

The private key → public key step is **one-way**:
- Easy: multiply private key by G to get public key
- Impossible in practice: given public key, find the private key
  (would take longer than the age of the universe with current computers)

---

## Step 2 — Hashing the Transaction

Before signing, the transaction data is **hashed**. You never sign the raw bytes
directly — you sign the hash of them.

```
  transaction bytes (all inputs, outputs, locktime, version...)
         │
         │  double SHA-256
         ▼
  tx_hash  ←── this 32-byte number is what gets signed
  = a single 256-bit number representing the entire transaction
```

This `tx_hash` is called the **message** in the signing algorithm.
If anyone changes even one byte of the transaction, `tx_hash` becomes completely
different, and the signature fails.

---

## Step 3 — Creating the Signature (ECDSA Signing)

Now you have:
- `private_key` = your private key (secret, never leaves wallet)
- `tx_hash` = hash of the transaction (derived from tx data)
- `generator` = the generator point G (public, hardcoded in secp256k1)

The ECDSA signing algorithm:

```
  1. Pick a random number called the signing_nonce.
     (ECDSA textbooks call this "k" — do NOT confuse with the mining nonce,
     completely different thing)
     Must be random and secret every single time.
     Reusing it leaks your private key. (This actually happened — PS3 was
     hacked because Sony reused the signing_nonce.)

  2. Calculate a random point on the curve:
     random_curve_point = signing_nonce × generator
     sig_r = random_curve_point.x_coordinate mod curve_order
             ← we only keep the x coordinate of that point

  3. Calculate the second half of the signature:
     sig_s = signing_nonce⁻¹ × (tx_hash + sig_r × private_key) mod curve_order
             ← signing_nonce⁻¹ means the modular inverse of signing_nonce

  4. The signature is: (sig_r, sig_s)
```

What each variable means in plain English:

```
  signing_nonce      = a fresh random number, used once, thrown away after
  random_curve_point = a random point on the curve derived from signing_nonce
  sig_r              = the x coordinate of random_curve_point (half of signature)
  sig_s              = a number that ties sig_r, tx_hash, and private_key together
                       (the other half of signature)
  curve_order        = the total number of points on the secp256k1 curve
                       (a fixed large prime, part of the standard)
```

Visually:

```
  ┌──────────────────────────────────────────────────────────┐
  │  INPUTS:                                                 │
  │    private_key   (secret, stays in wallet)               │
  │    tx_hash       (hash of all transaction data)          │
  │    signing_nonce (secret random number, fresh each time) │
  │    generator     (public hardcoded starting point)       │
  │                                                          │
  │  PROCESS:                                                │
  │    random_curve_point = signing_nonce × generator        │
  │    sig_r = random_curve_point.x_coordinate               │
  │    sig_s = signing_nonce⁻¹ × (tx_hash + sig_r           │
  │                                × private_key)            │
  │                                                          │
  │  OUTPUT:                                                 │
  │    signature = (sig_r, sig_s)                            │
  │    encoded as ~71 bytes in DER format                    │
  └──────────────────────────────────────────────────────────┘
```

The private_key and signing_nonce are **consumed internally** — they never appear
in the output. Only `(sig_r, sig_s)` comes out.

---

## Step 4 — Building scriptSig

Once the signature `(sig_r, sig_s)` is produced, the wallet assembles `scriptSig`:

```
  scriptSig:
  ┌──────────────────────────────────────────────────────────────┐
  │  <signature>  = (sig_r, sig_s) encoded in DER format (~71 B) │
  │  <public key> = compressed public key (33 bytes)             │
  └──────────────────────────────────────────────────────────────┘

  Total: ~104 bytes placed into the input of your transaction
```

This is the only moment the public key is revealed publicly.
It was never on the blockchain before (only its hash was, inside scriptPubKey).

---

## Step 5 — Verification (done by every node)

The node receives your transaction and verifies the signature using only
public information — no private key needed:

```
  INPUTS TO VERIFICATION:
    sig_r, sig_s   ← the two signature numbers from scriptSig
    signer_pubkey  ← the public key provided in scriptSig
    tx_hash        ← node recomputes this from the transaction data
    generator      ← the hardcoded secp256k1 starting point

  PROCESS:
    sig_s_inverse  = modular inverse of sig_s
    hash_scalar    = tx_hash × sig_s_inverse  mod curve_order
                     (scales tx_hash by the inverse of sig_s)
    key_scalar     = sig_r × sig_s_inverse    mod curve_order
                     (scales sig_r by the inverse of sig_s)
    recovered_point = (hash_scalar × generator)
                    + (key_scalar  × signer_pubkey)

  CHECK:
    Does recovered_point.x_coordinate == sig_r ?
      YES → signature is valid ✓  (the math closed the loop)
      NO  → signature is invalid ✗ (wrong key or tampered tx)
```

What each variable means in plain English:

```
  sig_s_inverse   = the "undo" of sig_s in modular arithmetic
  hash_scalar     = a number that represents the tx_hash's
                    contribution to the recovered point
  key_scalar      = a number that represents sig_r's
                    contribution to the recovered point
  recovered_point = a point on the curve computed entirely from
                    public information — if the signature is valid,
                    its x coordinate will exactly equal sig_r
```

Visually what this proves:

```
  ┌──────────────────────────────────────────────────────────┐
  │  If verification passes, it means:                       │
  │                                                          │
  │  1. The signer knew the private key matching             │
  │     signer_pubkey — because only that private key        │
  │     could produce a (sig_r, sig_s) that makes            │
  │     recovered_point.x_coordinate == sig_r                │
  │                                                          │
  │  2. The transaction was not tampered with —              │
  │     tx_hash is baked into sig_s during signing           │
  │     Change one byte of the tx → different tx_hash        │
  │     → recovered_point lands somewhere else               │
  │     → x coordinate no longer matches sig_r → FAIL        │
  └──────────────────────────────────────────────────────────┘
```

---

## The Full Picture — Everything Together

```
  YOUR WALLET
  ┌──────────────────────────────────────────────────────────┐
  │                                                          │
  │  private_key  ───────────────────────────────────────┐  │
  │       │                                              │  │
  │       │ × generator                                  │  │
  │       ▼                                              │  │
  │  public_key                                          │  │
  │       │                           SIGNING            │  │
  │       │ SHA-256 → RIPEMD-160      tx_hash ───────┐  │  │
  │       ▼                           signing_nonce ─┐│  │  │
  │  address (pubkey_hash)            private_key   ─┼┼──┘  │
  │       │                                          ││     │
  │       │                                          ▼▼     │
  │       │                          signature (sig_r, sig_s)│
  │       │                                      │         │
  └───────┼──────────────────────────────────────┼─────────┘
          │                                      │
          │                                      │
          ▼                                      ▼
  BLOCKCHAIN                             BLOCKCHAIN
  ┌──────────────────────┐        ┌──────────────────────────┐
  │  OUTPUT (received)   │        │  INPUT (when spending)   │
  │  scriptPubKey:       │        │  scriptSig:              │
  │  <address>           │        │  <signature (r,s)>       │
  │  (hash of pub key)   │        │  <public key>            │
  └──────────────────────┘        └──────────────────────────┘
  written by sender               written by you when spending
  private key never appears       private key never appears
```

---

## scriptSig vs scriptPubKey — Final Clear Summary

```
  scriptPubKey
  ┌──────────────────────────────────────────────────────────┐
  │  WHERE:    inside every OUTPUT on the blockchain         │
  │  WRITTEN:  by the SENDER when sending you money          │
  │  CONTAINS: hash of your public key (= your address)      │
  │  MEANING:  "only the owner of this address can spend"    │
  │  PRIVATE KEY INVOLVED: NO                                │
  └──────────────────────────────────────────────────────────┘

  scriptSig
  ┌──────────────────────────────────────────────────────────┐
  │  WHERE:    inside every INPUT on the blockchain          │
  │  WRITTEN:  by YOU when spending a UTXO                   │
  │  CONTAINS: your signature (r, s) + your public key       │
  │  MEANING:  "here is proof I own the address above"       │
  │  PRIVATE KEY INVOLVED: used to CREATE signature,         │
  │                        never stored or transmitted       │
  └──────────────────────────────────────────────────────────┘
```

---

## Why the Private Key Is Safe

The private key is used in the signing math but never appears in the output.
The only way to extract the private key from a signature would be to solve:

```
  sig_s = signing_nonce⁻¹ × (tx_hash + sig_r × private_key)
```

For `private_key` — but you do not know `nonce`, and finding it from `r`
requires solving the **elliptic curve discrete logarithm problem**, which is
computationally infeasible with current (and foreseeable) hardware.

This is the mathematical foundation that secures every Bitcoin transaction.

---

## Summary

```
A signature is a pair of numbers (sig_r, sig_s) produced by:
  1. Hashing the transaction data → tx_hash
  2. Picking a fresh random signing_nonce
  3. Computing random_curve_point = signing_nonce × generator
     sig_r = random_curve_point.x_coordinate
  4. Computing sig_s = signing_nonce⁻¹ × (tx_hash + sig_r × private_key)

scriptSig    = (sig_r, sig_s) + signer_pubkey
               lives in the INPUT of a spending transaction
               private_key is used to produce it but never included

scriptPubKey = pubkey_hash (address) + verification opcodes
               lives in the OUTPUT when money is sent to you
               written by the sender, not you

Verification = node computes:
               sig_s_inverse   = modular inverse of sig_s
               hash_scalar     = tx_hash × sig_s_inverse
               key_scalar      = sig_r  × sig_s_inverse
               recovered_point = hash_scalar × generator
                               + key_scalar  × signer_pubkey
               valid if recovered_point.x_coordinate == sig_r

               private_key is never needed, never leaves your wallet
```
