# Soroban Multi-Asset AMM Liquidity Pool

A decentralized **Automated Market Maker (AMM)** smart contract built on Stellar's [Soroban](https://soroban.stellar.org/) network. This contract implements the constant product formula ($x \times y = k$) to enable permissionless token swapping between any two supported Stellar Classic or Soroban-native assets, while issuing and tracking Liquidity Provider (LP) shares.

---

## Features

- **Constant Product AMM** — Swap tokens using the $x \times y = k$ invariant with a 0.3% protocol fee.
- **Fixed-Point Math** — All arithmetic uses `u128` integer math (`#![no_std]`), as Soroban does not support floating-point.
- **Slippage Protection** — Accepts a `min_out` parameter to prevent front-running and excessive slippage.
- **LP Token Lifecycle** — Mint and burn liquidity pool share tokens on deposit and withdrawal; tracks total LP supply in persistent state.
- **Security Hardening** — Guards against zero-liquidity edge cases and division-by-zero panics.
- **CI/CD Automation** — Pre-configured GitHub Actions workflow that runs `cargo fmt`, `cargo clippy`, and `cargo test` on every PR.

---

## Active Issues & Point Matrix

Issues are assigned point values based on complexity. Check the wave tags in the issue tracker before assigning yourself.

| Complexity | Points | Issue | Description |
|------------|--------|-------|-------------|
| 🔴 High | **200** | [#58](<issue-link>) | Implement Constant Product Swap Math & Slippage Protection |
| 🟡 Medium | **150** | [#62](<issue-link>) | LP Token Minting & Burning Lifecycle |
| 🟢 Trivial | **100** | [#65](<issue-link>) | Standardize CI/CD Testing Actions |

### 🔴 High — Issue #58: Constant Product Swap Math & Slippage Protection

Write the core token swap logic using native `#![no_std]` Rust math. The contract must calculate the output token amount minus a **0.3% protocol fee**, while accepting a `min_out` parameter to prevent front-running and excessive slippage.

**Requirements:**
- Implement fixed-point math using `u128` (no floating-point).
- Unit tests demonstrating the invariant ($x \times y = k$) holds after every swap.

### 🟡 Medium — Issue #62: LP Token Minting & Burning Lifecycle

Implement the logic to mint liquidity pool share tokens on deposit and burn them on withdrawal. Update persistent state to track total LP token supply.

**Requirements:**
- Prevent zero-liquidity edge cases that could cause division-by-zero panics.
- Test that users receive the mathematically correct proportion of pool fees upon withdrawal.

### 🟢 Trivial — Issue #65: Standardize CI/CD Testing Actions

Update `.github/workflows/rust.yml` to automate testing for the Soroban contract. The action must run `cargo fmt --check`, `cargo clippy`, and `cargo test --features testutils` on every PR opened against `main`.

**Requirements:**
- The workflow must strictly use the `wasm32-unknown-unknown` target.

---

## Drips Wave Program

This repository uses the **Drips Wave Program** to fund community contributions through recurring 1-week sprints. Browse the backlog, complete issues, and earn a share of the monthly reward pool via the *Fix, Merge, and Earn* workflow.

> **⚠️ Pre-Requisite:** To participate and receive points for your PRs, you must complete KYC identity verification in the [Drips App](https://drips.network/). This is a strict regulatory requirement for withdrawing funds and ensures Sybil resistance.

---

## Maintainer Acceptance Criteria

- **Test Coverage** — All math logic and state changes must achieve **95% test coverage** using the Soroban Env mocking framework.
- **AI Contribution Policy** — AI tools may assist your workflow, but submitting untested, low-quality code you do not fundamentally understand is a **Prohibited Activity** under Drips Wave terms. PRs must pass rigorous code review.
- **PR Linking** — Pull request descriptions must clearly link the original issue (e.g., `Resolves #58`) so the Drips indexer can accurately award points upon merge.

---

## Local Development Setup

### Prerequisites

- Rust toolchain (latest stable)
- `wasm32-unknown-unknown` target installed
- Soroban CLI

### Quick Start

```bash
git clone https://github.com/yourusername/soroban-amm-pool.git
cd soroban-amm-pool
cargo build --target wasm32-unknown-unknown --release
cargo test
```
