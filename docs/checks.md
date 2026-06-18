# Checks reference

This document describes what each Soroban Guard Core check looks for and why it matters.

---

## `missing-require-auth` (High)

**Status:** Phase 1

**What it detects**

In an `impl` block marked with `#[contractimpl]` or `#[soroban_sdk::contractimpl]`, any function whose body:

1. Performs a storage mutation through `env.storage()` (heuristic: method calls `set`, `remove`, `extend_ttl`, `bump`, or `append` on a receiver chain that includes `.storage()`), and  
2. Never calls `env.require_auth()` (parameter name **`env`**: `env.require_auth()`).

**Why it matters**

Contract state updates should be gated. This rule only recognizes `env.require_auth()`, not `user.require_auth()` or `env.require_auth_for_args()`.

**Limitations**

- Only the `Env` binding named `env` counts.
- Static analysis cannot see auth hidden in helpers.

**Fixture:** `test-contracts/vulnerable/`, `test-contracts/safe/`

---

## `unchecked-arithmetic` (Medium)

**Status:** Phase 2

**What it detects**

Inside `#[contractimpl]` methods:

- Binary `+`, `-`, `*` where **both** sides are not integer/string literals (so `1 + 2` is ignored, `a + b` is flagged).
- Compound `+=`, `-=`, `*=` (syn 2 represents these as `ExprBinary` with `AddAssign` / `SubAssign` / `MulAssign`).

**Why it matters**

Wrapping arithmetic on `i128` / `u128` amounts can silently overflow. Prefer `checked_*` or `saturating_*` for token math.

**Limitations**

- May flag harmless loop indices; review context.
- Does not analyze types; it is syntactic.

**Fixture:** `test-contracts/arithmetic-vulnerable/`, `test-contracts/arithmetic-safe/`

---

## `unprotected-admin` (High)

**Status:** Phase 2

**What it detects**

Public (`pub fn`) methods in `#[contractimpl]` whose name **exactly matches** a built-in list of sensitive entrypoints (e.g. `set_owner`, `pause`, `migrate`, `upgrade`, … — see `SENSITIVE_NAMES` in `crates/checks/src/admin.rs`), and whose body contains **no** call to `require_auth` or `require_auth_for_args` on any receiver.

**Why it matters**

Names like `set_owner` strongly suggest privilege; without any auth call the scanner treats the entrypoint as world-callable.

**Limitations**

- Name allowlist only; extend the list as your org sees fit.
- Any `require_auth` / `require_auth_for_args` anywhere in the body clears the finding (no dataflow).

**Fixture:** `test-contracts/admin-vulnerable/`, `test-contracts/admin-safe/`

---

## `unsafe-storage-patterns` (Medium)

**Status:** Phase 2

**What it detects**

1. **Temporary storage writes** — `env.storage().temporary()` in the receiver chain of a storage mutation (`set`, `remove`, `extend_ttl`, `bump`, `append`).
2. **Dynamic `Symbol::new` keys** — `Symbol::new(&env, …)` where the second argument is **not** a string literal (e.g. derived from a parameter). Literal second args like `Symbol::new(&env, "fixed")` are ignored.

**Why it matters**

- Temporary data expires with TTL; it is easy to misuse for long-lived balances or ownership.
- Caller-derived symbol strings are easier to enumerate or collide than fixed `symbol_short!` keys.

**Limitations**

- Does not analyze `symbol_short!(...)` macros beyond normal parsing.
- `Symbol::new` with a `const` or macro-expanded literal may still be flagged if it is not a `syn::Lit::Str`.

**Fixture:** `test-contracts/storage-vulnerable/`, `test-contracts/storage-safe/`

---

## `instance-ttl-missing` (Medium)

**Status:** Phase 1

**What it detects**

In a contract file, if there is at least one call to `env.storage().instance().set(...)` but no call to `env.storage().instance().extend_ttl(...)` anywhere in the file.

**Why it matters**

Instance storage in Soroban has a TTL (time-to-live) and will expire if not periodically extended. If a contract uses instance storage but never extends its TTL, the contract may become inaccessible once the instance expires.

**Limitations**

- Only detects direct calls; does not analyze indirect calls through helper functions.
- Checks the entire file, not per function.

**Fixture:** `test-contracts/instance-ttl-vulnerable/`, `test-contracts/instance-ttl-safe/`

---

## `storage-key-collision` (Medium)

**Status:** Phase 1

**What it detects**

Storage keys with similar names that could lead to accidental overwrites, such as "owner", "owner_addr", and "owner_address" in the same contract.

**Why it matters**

Similar key names can cause developers to accidentally use the wrong key when reading or writing storage, leading to data corruption or security vulnerabilities. Distinct key names help prevent these mistakes.

**Limitations**

- Only detects string literal keys, not symbol-based keys
- May flag some legitimate cases where similar keys are intentionally used

**Fixture:** `test-contracts/storage-key-collision-vulnerable/`, `test-contracts/storage-key-collision-safe/`

---

## `zero-divisor` (High)

**Status:** Phase 2

**What it detects**

Inside `#[contractimpl]` methods, any `/` (division) or `%` (remainder) where the right-hand operand is a function parameter and the method body does **not** contain a zero-check guard for that parameter anywhere.

A guard is recognized as:

- `assert!(param ...)` — an `assert!` macro whose token stream contains the parameter name (textual heuristic).
- `if cond { ... }` — an `if` expression whose condition contains both the parameter name and the literal `"0"`.

**Why it matters**

Integer division or remainder by zero causes a panic in Rust, which terminates the entire Soroban transaction. An attacker who controls any fee, rate, or price argument can pass `0` to permanently brick any entrypoint that divides by that parameter without checking for zero first.

**Limitations**

- Syntactic, not type-aware: any parameter matching the name triggers the finding; the check does not verify the parameter is actually a numeric type.
- Guards are detected by substring match anywhere in the body, not by dataflow.
- `assert_eq!(param, 0)` (two-argument form) is not recognized — only the single-argument `assert!` form counts.

**Fixture:** `test-contracts/zero-divisor-vulnerable/`, `test-contracts/zero-divisor-safe/`
