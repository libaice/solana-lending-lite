## Solana Lending Protocol

A non-custodial lending protocol built on Solana using the [Anchor framework](https://github.com/solana-foundation/anchor).

This protocol enables users to supply assets to earn interest and to borrow other assets by providing collateral. 

It is inspired by the fundamental mechanisms of protocols like Compound and Aave, but tailored for Solana's speed and composability.



---



### ⚙️ Core Features

> - Collateralized lending with configurable Loan-to-Value (LTV)
> - Interest accrual over time using a simple linear model
> - Liquidation system with configurable bonuses
> - Secure token vaults managed by PDAs
> - Oracle integration for real-time asset pricing





---



### 🧱 Protocol Architecture

**Key Accounts**

| Account      | Description                                               |
| ------------ | --------------------------------------------------------- |
| `Reserve`    | Stores configuration, liquidity data, interest parameters |
| `Obligation` | Tracks a user’s deposits, borrowings, and health factor   |
| `Vault`      | PDA-controlled account holding actual token balances      |
| `Oracle`     | External source for asset price (e.g., Pyth, Switchboard) |



**Instruction Set**

- `InitializeReserve`: Create a new lending market for a supported token
- `Deposit`: Supply assets and mint interest-bearing credits
- `Borrow`: Borrow assets by locking collateral under LTV limits
- `Repay`: Repay borrowed assets and update interest
- `Withdraw`: Withdraw supplied collateral when not borrowed
- `Liquidate`: Liquidate under-collateralized positions





---



### 📊 Sample Parameters

| Parameter          | Value   |
| ------------------ | ------- |
| Borrow Factor      | 60%     |
| Liquidation LTV    | 75%     |
| Liquidation Bonus  | 5% ~10% |
| Base Interest Rate | 3% APR  |



---



### 🧪 Tech Stack

- [**Solana** ](https://github.com/solana-labs/solana)– High-throughput L1 blockchain
- [**Anchor**](https://github.com/solana-foundation/anchor) – Framework for Solana smart contract development
- [**Rust** ](https://github.com/rust-lang/rust)– Core language for writing on-chain logic
- [**Pyth Network**](https://github.com/pyth-network) – Oracle solution for price feeds
- [**TypeScript** ](https://www.anchor-lang.com/docs/clients/typescript)– Optional frontend / SDK integration



---



### 🚀 Deployment Guide

> Make sure `anchor`, `solana-cli`, and Rust toolchain are properly installed.

```bash
# Clone the repo
git clone https://github.com/libaice/solana-lending-lite
cd solana-lending-lite

# Build and deploy
anchor keys sync
anchor build
anchor deploy --provider.cluster devnet
