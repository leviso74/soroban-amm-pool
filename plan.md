# Soroban AMM Pool — Wave Sprint Plan

## Project Overview
A `#![no_std]` Rust smart contract on Stellar Soroban implementing a constant-product AMM (x × y = k) with fixed-point i128 math, LP token mint/burn lifecycle, and slippage-protected swaps.

## Wave Sprint Structure
Seven-day synchronized sprints where contributors implement contract logic, harden arithmetic, and expand test coverage. Points redeemable for on-chain rewards.

## Issue Types & Point Matrix

### Bug Fixes (100–200 pts)
- **Integer overflow on large swaps** (200 pts) — When reserve or input amounts approach i128::MAX, intermediate multiplication overflows silently. Audit all `checked_mul`/`checked_add` paths and add fuzz-proof overflow tests.
- **Zero-liquidity withdraw panic** (150 pts) — Calling `withdraw` with zero LP tokens panics instead of returning a clear error. Add early guard with descriptive `require!` message.
- **Fee rounding asymmetry** (150 pts) — The protocol fee (0.3%) is rounded down during swap-in but the invariant check allows a 1-wei imbalance. Implement banker's rounding and prove k never decreases.
- **Address validation in initialize** (100 pts) — `initialize` accepts non-contract addresses. Add `require_auth` check or verify addresses have code via the Soroban host function.

### New Features (100–200 pts)
- **Multi-hop swap routing** (200 pts) — Allow swapping between token A and token C via an intermediate pool B, splitting the route for optimal price.
- **Flash loans** (200 pts) — Implement ERC-3156-style flash loans: borrow reserves, execute arbitrary logic in one transaction, and repay with a 0.09% fee or revert.
- **Dynamic fee tiers** (150 pts) — Governance-controlled fee switch that adjusts the protocol fee from 0.05% (stable pairs) to 1% (exotic pairs) based on pool volatility.
- **LP token metadata** (100 pts) — Add `name()`, `symbol()`, and `decimals()` read functions to the LP token for wallet compatibility.
- **Pause mechanism** (100 pts) — Owner-only pause/unpause that halts swaps but allows withdrawals, for emergency incident response.

### Documentation (50–100 pts)
- **Soroban contract spec** (100 pts) — Publish a formal specification using Soroban's built-in spec generation, covering all public functions, parameter types, and error codes.
- **Integration guide** (50 pts) — Step-by-step for frontend devs: how to call `swap_exact_a_for_b` from TypeScript using `soroban-client`.
- **Math deep-dive** (50 pts) — Explain the fixed-point derivation, fee mechanics, and LP token valuation in a referenced markdown doc.

### Testing (100–150 pts)
- **Property-based invariant tests** (150 pts) — Use `proptest` to generate random reserves, swap amounts, and fee tiers; verify k never decreases and LP share ratios remain consistent.
- **Edge case suite** (100 pts) — Test minimum swap amounts (1 unit), maximum reserves, simultaneous deposits/withdraws, and reentrancy scenarios.
- **Soroban env integration tests** (150 pts) — Full contract deployment and interaction test using `SorobanEnv` mock, verifying emitted events match expected values.

## Acceptance Criteria
- 95% branch coverage via `cargo tarpaulin` or `llvm-cov`
- All arithmetic paths use `checked_*` methods — no raw `+`, `-`, `*`, `/`
- Events emitted on every state-changing call for off-chain indexing
- PR must link the original issue (e.g., `Resolves #72`)
