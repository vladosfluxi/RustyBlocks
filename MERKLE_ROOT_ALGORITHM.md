# Merkle Root Algorithm — Building a Tree from Leaves

## What is a Merkle Tree?

A Merkle tree is a binary hash tree. Every leaf node holds the hash of a piece of data. Every internal (parent) node holds the hash of the concatenation of its two children's hashes. This process repeats upward until only one hash remains — the **Merkle Root**.

The Merkle Root is a single fingerprint that represents all data in the tree. If any leaf changes, the root changes.

---

## Core Concepts Before We Begin

### Leaf
A leaf is the bottom-most element of the tree. Each leaf holds the hash of a single piece of data (e.g., a transaction).

```
Leaf = hash(data)
```

### Internal Node
An internal node holds the hash of its two children concatenated together.

```
Node = hash(left_child || right_child)
```

The `||` symbol means concatenation — you join the two hashes byte-by-byte, then hash the result.

### Root
The single node at the top of the tree. There is always exactly one root.

---

## The Fundamental Pairing Rule

**Leaves and nodes are always paired from left to right.**

Given a list of items, you take them two at a time:
- Item 1 pairs with Item 2
- Item 3 pairs with Item 4
- Item 5 pairs with Item 6
- ...and so on

The result of each pair becomes one node on the next level up. Then you repeat the pairing on that new level, until only one node remains.

---

## The Odd Number Rule (Duplication)

When a level has an **odd number** of elements, you cannot pair the last element with a partner — there is none. The solution is to **duplicate the last element** and pair it with itself.

```
[A, B, C]  →  pair (A,B) and (C,C)
              → [Node(A,B), Node(C,C)]
```

This is the standard Bitcoin-style Merkle tree behavior. The duplicate does not change the data — it is a structural necessity to keep the tree binary.

---

## Step-by-Step Algorithm

```
1. Start with a list of leaf hashes.
2. If the list has only 1 element, that element IS the Merkle Root. Stop.
3. If the list has an odd number of elements, duplicate the last element.
4. Pair up elements: (0,1), (2,3), (4,5), ...
5. For each pair, compute: hash(left || right)
6. Collect all results into a new list. This is the next level up.
7. Go back to step 2 and repeat with the new list.
```

---

## Example 1 — Four Leaves (Perfect Tree)

Four leaves is the simplest case because 4 is a power of 2. No duplication is needed at any level.

### Starting Leaves

```
Data:    [TX_A,   TX_B,   TX_C,   TX_D  ]
Hashes:  [H(A),   H(B),   H(C),   H(D)  ]
```

### Level 0 — The Leaves

```
 [  H(A)      H(B)      H(C)      H(D)  ]
     0          1          2          3
```

There are 4 items. 4 is even, so no duplication needed.

### Pairing on Level 0

Pair indices (0,1) and (2,3):

```
Pair 1:  H(A) and H(B)  →  hash( H(A) || H(B) )  =  H(AB)
Pair 2:  H(C) and H(D)  →  hash( H(C) || H(D) )  =  H(CD)
```

### Level 1 — Two Nodes

```
 [  H(AB)             H(CD)  ]
     0                   1
```

There are 2 items. 2 is even, so no duplication needed.

### Pairing on Level 1

Pair indices (0,1):

```
Pair 1:  H(AB) and H(CD)  →  hash( H(AB) || H(CD) )  =  H(ABCD)
```

### Level 2 — The Root

```
 [  H(ABCD)  ]
```

Only one item remains. **This is the Merkle Root.**

### Full Tree Diagram

```
                    [ H(ABCD) ]          ← ROOT
                   /           \
            [ H(AB) ]       [ H(CD) ]   ← Level 1
            /       \       /       \
         H(A)     H(B) | H(C)     H(D)  ← Level 0 (Leaves)
```

---

## Example 2 — Two Leaves (Minimal Non-Trivial Tree)

### Starting Leaves

```
Data:    [TX_A,   TX_B  ]
Hashes:  [H(A),   H(B)  ]
```

### Level 0 — The Leaves

```
 [  H(A)      H(B)  ]
     0          1
```

Two items. Even. No duplication.

### Pairing on Level 0

```
Pair 1:  H(A) and H(B)  →  hash( H(A) || H(B) )  =  H(AB)
```

### Level 1 — The Root

```
 [  H(AB)  ]
```

**This is the Merkle Root.**

### Full Tree Diagram

```
        [ H(AB) ]       ← ROOT
        /        \
     H(A)       H(B)    ← Level 0 (Leaves)
```

---

## Example 3 — One Leaf (Degenerate Case)

When there is only one transaction, there is no tree to build. The single leaf hash is the root.

```
Data:    [TX_A]
Hashes:  [H(A)]
```

```
 [ H(A) ]   ← ROOT (also the only leaf)
```

No pairing. No levels. **H(A) is the Merkle Root.**

---

## Example 4 — Three Leaves (Odd Number, Duplication Required)

This is where the odd-number rule kicks in for the first time.

### Starting Leaves

```
Data:    [TX_A,   TX_B,   TX_C  ]
Hashes:  [H(A),   H(B),   H(C)  ]
```

### Level 0 — The Leaves

```
 [  H(A)      H(B)      H(C)  ]
     0          1          2
```

Three items. **3 is ODD.** Duplicate the last element:

```
 [  H(A)      H(B)      H(C)      H(C)  ]
     0          1          2          3
                                   ^^^^
                             (duplicated last item)
```

### Pairing on Level 0

```
Pair 1:  H(A) and H(B)  →  hash( H(A) || H(B) )  =  H(AB)
Pair 2:  H(C) and H(C)  →  hash( H(C) || H(C) )  =  H(CC)
```

### Level 1 — Two Nodes

```
 [  H(AB)      H(CC)  ]
     0            1
```

Two items. Even. No duplication.

### Pairing on Level 1

```
Pair 1:  H(AB) and H(CC)  →  hash( H(AB) || H(CC) )  =  H(ABCC)
```

### Level 2 — The Root

```
 [  H(ABCC)  ]
```

**This is the Merkle Root.**

### Full Tree Diagram

```
                  [ H(ABCC) ]              ← ROOT
                 /            \
           [ H(AB) ]       [ H(CC) ]       ← Level 1
           /       \       /       \
        H(A)     H(B) | H(C)     H(C)*    ← Level 0 (Leaves)
                                    *duplicated
```

Notice: the right branch of the tree is `H(C)` paired with itself. This is completely valid. It simply means "C appears twice in the structural sense" — but the actual data only has TX_A, TX_B, TX_C.

---

## Example 5 — Five Leaves (Odd Number, Multi-Level)

This example shows duplication happening on one level, then a clean even number on the next.

### Starting Leaves

```
Data:    [TX_A,  TX_B,  TX_C,  TX_D,  TX_E ]
Hashes:  [H(A),  H(B),  H(C),  H(D),  H(E) ]
```

### Level 0 — The Leaves

```
 [  H(A)    H(B)    H(C)    H(D)    H(E)  ]
     0        1       2       3       4
```

Five items. **5 is ODD.** Duplicate the last:

```
 [  H(A)    H(B)    H(C)    H(D)    H(E)    H(E)  ]
     0        1       2       3       4        5
                                             ^^^^
                                          (duplicated)
```

### Pairing on Level 0

```
Pair 1:  H(A) and H(B)  →  H(AB)
Pair 2:  H(C) and H(D)  →  H(CD)
Pair 3:  H(E) and H(E)  →  H(EE)
```

### Level 1 — Three Nodes

```
 [  H(AB)    H(CD)    H(EE)  ]
     0          1        2
```

Three items. **3 is ODD.** Duplicate the last:

```
 [  H(AB)    H(CD)    H(EE)    H(EE)  ]
     0          1        2        3
                                ^^^^
                            (duplicated)
```

### Pairing on Level 1

```
Pair 1:  H(AB)  and H(CD)   →  H(ABCD)
Pair 2:  H(EE)  and H(EE)   →  H(EEEE)
```

### Level 2 — Two Nodes

```
 [  H(ABCD)    H(EEEE)  ]
     0            1
```

Two items. Even. No duplication.

### Pairing on Level 2

```
Pair 1:  H(ABCD) and H(EEEE)  →  H(ABCDEEEE)
```

### Level 3 — The Root

```
 [  H(ABCDEEEE)  ]
```

**This is the Merkle Root.**

### Full Tree Diagram

```
                        [ H(ABCDEEEE) ]                    ← ROOT
                       /               \
               [ H(ABCD) ]          [ H(EEEE) ]            ← Level 2
               /          \          /         \
          [ H(AB) ]   [ H(CD) ]  [ H(EE) ]  [ H(EE) ]*    ← Level 1
          /      \    /      \    /      \
       H(A)   H(B) H(C)   H(D) H(E)   H(E)*               ← Level 0
                                         *duplicated
```

Two duplications occurred: one at Level 0 (H(E)), and one at Level 1 (H(EE)).

---

## Example 6 — Eight Leaves (Perfect Power-of-Two Tree)

Eight leaves form a perfectly balanced binary tree with no duplication at any level.

### Starting Leaves

```
 [H(A), H(B), H(C), H(D), H(E), H(F), H(G), H(H)]
```

### Level 0 — Eight Leaves

```
 [ H(A)  H(B) | H(C)  H(D) | H(E)  H(F) | H(G)  H(H) ]
```

### Pairing on Level 0

```
H(A)+H(B)  →  H(AB)
H(C)+H(D)  →  H(CD)
H(E)+H(F)  →  H(EF)
H(G)+H(H)  →  H(GH)
```

### Level 1 — Four Nodes

```
 [ H(AB)  H(CD) | H(EF)  H(GH) ]
```

### Pairing on Level 1

```
H(AB)+H(CD)  →  H(ABCD)
H(EF)+H(GH)  →  H(EFGH)
```

### Level 2 — Two Nodes

```
 [ H(ABCD)   H(EFGH) ]
```

### Pairing on Level 2

```
H(ABCD)+H(EFGH)  →  H(ABCDEFGH)
```

### Level 3 — The Root

```
 [ H(ABCDEFGH) ]
```

### Full Tree Diagram

```
                              [ H(ABCDEFGH) ]                         ← ROOT
                             /               \
                   [ H(ABCD) ]            [ H(EFGH) ]                 ← Level 2
                  /           \           /          \
            [H(AB)]        [H(CD)]   [H(EF)]      [H(GH)]            ← Level 1
            /    \          /    \    /    \        /    \
          H(A) H(B)      H(C) H(D) H(E) H(F)    H(G) H(H)           ← Level 0
```

Clean and symmetric. No duplication needed anywhere.

---

## Example 7 — Six Leaves

Six leaves: an even number at Level 0, but then an odd number at Level 1.

### Starting Leaves

```
 [H(A), H(B), H(C), H(D), H(E), H(F)]
```

### Level 0 — Six Leaves

```
 [ H(A)  H(B) | H(C)  H(D) | H(E)  H(F) ]
```

Six items. Even. No duplication.

### Pairing on Level 0

```
H(A)+H(B)  →  H(AB)
H(C)+H(D)  →  H(CD)
H(E)+H(F)  →  H(EF)
```

### Level 1 — Three Nodes

```
 [ H(AB)    H(CD)    H(EF) ]
     0          1       2
```

**3 is ODD.** Duplicate the last:

```
 [ H(AB)    H(CD)    H(EF)    H(EF) ]
     0          1       2        3
```

### Pairing on Level 1

```
H(AB)+H(CD)   →  H(ABCD)
H(EF)+H(EF)   →  H(EFEF)
```

### Level 2 — Two Nodes

```
 [ H(ABCD)   H(EFEF) ]
```

### Pairing on Level 2

```
H(ABCD)+H(EFEF)  →  H(ABCDEFEF)
```

### Level 3 — The Root

**H(ABCDEFEF) is the Merkle Root.**

### Full Tree Diagram

```
                     [ H(ABCDEFEF) ]                        ← ROOT
                    /               \
            [ H(ABCD) ]          [ H(EFEF) ]                ← Level 2
            /          \          /         \
       [ H(AB) ]   [ H(CD) ]  [ H(EF) ]  [ H(EF) ]*        ← Level 1
       /      \    /      \    /      \
    H(A)   H(B) H(C)   H(D) H(E)   H(F)                    ← Level 0
```

Duplication happened only at Level 1, not Level 0.

---

## How Many Levels Will The Tree Have?

The number of levels (including the root) is:

```
levels = ceil( log2(number_of_leaves) ) + 1
```

| Leaves | Levels |
|--------|--------|
| 1      | 1      |
| 2      | 2      |
| 3–4    | 3      |
| 5–8    | 4      |
| 9–16   | 5      |
| 17–32  | 6      |

With duplication, odd trees require the same number of levels as the next even number.

---

## Concatenation Order Matters

When you hash a pair, the **order** must be consistent: always `hash(left || right)`, never `hash(right || left)`. Swapping the order produces a completely different hash.

```
hash( H(A) || H(B) )  ≠  hash( H(B) || H(A) )
```

In a Merkle tree, "left" is always the element with the **lower index** in the current level's list, and "right" is the element with the **higher index**.

---

## Summary of the Full Grouping Process

```
Given:  [L0, L1, L2, L3, L4, L5, L6, L7]    (8 leaves, all even — ideal case)

Level 0 (leaves):
  Pairs:   (L0, L1)   (L2, L3)   (L4, L5)   (L6, L7)
  Results:  N0          N1          N2          N3

Level 1:
  Pairs:   (N0, N1)   (N2, N3)
  Results:  N4          N5

Level 2:
  Pairs:   (N4, N5)
  Results:  ROOT

Done.
```

---

## Summary of the Odd-Number Duplication Process

```
Given:  [L0, L1, L2]    (3 leaves — odd)

Level 0 (leaves):
  Odd count detected! Duplicate L2:
  [L0, L1, L2, L2]
  Pairs:   (L0, L1)   (L2, L2)
  Results:  N0          N1

Level 1:
  Even count. No duplication.
  Pairs:   (N0, N1)
  Results:  ROOT

Done.
```

---

## The Invariant to Remember

> At every level, you always process the list left to right, two items at a time. If there is a leftover item at the end, clone it to make a pair. The output list is always half the size (rounded up) of the input list. Repeat until the list has one item.

That one remaining item is your **Merkle Root**.
