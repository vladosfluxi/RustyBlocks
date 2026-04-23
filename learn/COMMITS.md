# How to Write Commit Messages Like a Pro

---

## The Golden Rule

A commit message answers two questions:

1. **What** does this commit do?
2. **Why** is it being done?

If you can't answer both, the commit isn't ready.

---

## The Conventional Commits Standard

Most professional teams follow this format:

```
<type>(<scope>): <short summary>

<optional body — explains the WHY>

<optional footer — references issues, breaking changes>
```

### Types (use these prefixes)

| Type       | When to use                                              |
|------------|----------------------------------------------------------|
| `feat`     | A new feature for the user                               |
| `fix`      | A bug fix                                                |
| `refactor` | Code change that neither fixes a bug nor adds a feature  |
| `perf`     | A code change that improves performance                  |
| `style`    | Formatting, whitespace, semicolons (no code change)      |
| `docs`     | Documentation only                                       |
| `test`     | Adding or updating tests                                 |
| `chore`    | Build process, dependencies, tooling (not user-facing)   |
| `revert`   | Reverting a previous commit                              |

### Examples

```
feat(blockchain): add genesis block creation
fix(merkle): handle empty transaction list
refactor(block): extract header hashing into serialize()
docs(transactions): add UTXO explanation
test(mining): verify hash meets target after mining
chore(deps): bump num-bigint to 0.4.6
```

---

## The 50/72 Rule

- **First line: max 50 characters.** Forces you to be concise.
- **Body lines: wrap at 72 characters.** Plays well with `git log` in narrow terminals.
- **Blank line between summary and body.** Required.

```
fix(merkle): pad odd transaction lists by duplicating last tx

Previously the merkle tree would panic when given an odd number
of transactions. Bitcoin handles this by duplicating the final
transaction before pairing. This commit matches that behavior.
```

---

## Tense and Voice

**Use imperative mood** — like you're commanding the codebase:

```
✓ Add validation for block hashes
✓ Fix off-by-one error in nonce loop
✓ Remove unused chrono import

✗ Added validation for block hashes        (past tense)
✗ Adding validation for block hashes       (gerund)
✗ Adds validation for block hashes         (third person)
```

**Why imperative?** Because git itself uses it: "Merge branch X", "Revert commit Y".
Your commit slots in naturally with auto-generated messages.

---

## What NOT to Do

### ❌ Vague messages
```
update
fix
test
wip
changes
stuff
```
Future you will hate present you. Every commit should make sense in `git log` years later.

### ❌ Multiple unrelated changes in one commit
```
feat: add mining + fix typo + update README + delete old file
```
Split into 4 commits. One commit = one logical change.

### ❌ File paths in the title
```
✗ Add function calculate_target in src/blockchain.rs
✓ feat(blockchain): add target calculation
```
`git log` already shows the files. Don't waste space.

### ❌ Spelling and grammar mistakes
```
✗ Impelement genesis block        (typo: "Implement")
✗ funtion add_block_mining        (typo: "function")
```
These stay in history forever. Slow down and proofread.

### ❌ Emoji-only or meme commits in real projects
```
✗ 🔥💯 added the cool stuff
```
Cute for personal projects. Unprofessional in real codebases.

---

## When to Add a Body

Use the body when the WHY isn't obvious from the summary alone.

### Good — short, clear, no body needed:
```
fix(blockchain): include user txids in merkle root
```

### Better — body explains the WHY when it matters:
```
refactor(block): split into separate module files

The block.rs file had grown to handle transactions, merkle trees,
hashing, and the chain itself. Splitting these into separate files
clarifies ownership and prevents circular dependencies as more
features are added.
```

---

## Atomic Commits — One Change Per Commit

A commit should represent **one logical change**. Rules of thumb:

- If you'd say "and" while describing it, split it.
- Each commit should compile and pass tests on its own.
- Reverting one commit should not break unrelated features.

```
✓ Three separate commits:
  feat(blockchain): add genesis block
  feat(blockchain): add add_block method
  test(blockchain): verify chain growth

✗ One commit:
  feat: add genesis, add_block, and tests
```

This makes `git bisect`, code review, and rollbacks much easier.

---

## Examples — Bad vs. Good

| Bad                                          | Good                                                |
|----------------------------------------------|-----------------------------------------------------|
| `update`                                     | `fix(mining): break loop when hash meets target`   |
| `Feat: funtion add_block_mining`             | `feat(blockchain): add add_block method`            |
| `Impelement genesis block`                   | `feat(blockchain): create genesis block`            |
| `test`                                       | `test(merkle): verify root with 4 transactions`    |
| `Add things`                                 | `feat(transaction): add coinbase reward output`     |
| `fixed bug`                                  | `fix(merkle): pad odd-length tx list`               |

---

## A Real Pro Workflow

1. **Make small changes** — don't code for 3 hours then commit everything.
2. **Stage selectively** — `git add -p` lets you stage individual hunks, not whole files.
3. **Write the message thoughtfully** — pretend you're explaining to a stranger.
4. **Re-read before pushing** — typos in commit messages live forever.

---

## Quick Reference Card

```
<type>(<scope>): <imperative summary, max 50 chars>

<body — explain WHY, wrap at 72 chars>

<footer — issue refs, breaking changes>
```

Types: `feat | fix | refactor | perf | style | docs | test | chore | revert`

Rules:
- Imperative mood ("add" not "added")
- One logical change per commit
- No file paths in title
- Proofread before committing
- Body explains WHY, code shows WHAT

---

## TL;DR

Bad: `Feat: funtion add_block_mining src/blockchain.rs`
Good: `feat(blockchain): add add_block method`

A senior engineer should be able to read your `git log` and understand the project history without reading any code.
