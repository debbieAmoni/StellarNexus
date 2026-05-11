# 🏺 StellarNexus
> A "Dead Man's Switch" for the digital age. Secure your legacy with programmable inheritance on the Stellar blockchain.

[![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar-blue)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban-purple)](https://soroban.stellar.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## 🌌 What is StellarNexus?

StellarNexus is a programmable inheritance vault built on Stellar's Soroban smart contract platform. It solves one of crypto's most overlooked problems: **capital permanently locked in dead wallets**.

When an owner becomes incapacitated or passes away, their digital assets don't have to disappear. StellarNexus uses a heartbeat mechanism — a simple periodic transaction — to confirm the owner is still active. If the heartbeat stops, the vault automatically begins distributing assets to designated heirs via a drip stream.

This creates compounding **"Lindy Effect"** value: capital stays productive and circulating within the Stellar ecosystem across generations rather than being lost forever.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      StellarNexus                        │
│                                                         │
│  ┌──────────────┐    ┌──────────────┐   ┌────────────┐ │
│  │  Vault Core  │───▶│  Heartbeat   │──▶│  Timelock  │ │
│  │  (Storage)   │    │   Monitor    │   │   Engine   │ │
│  └──────────────┘    └──────────────┘   └─────┬──────┘ │
│                                               │        │
│                                               ▼        │
│                                    ┌──────────────────┐ │
│                                    │   Drip Stream    │ │
│                                    │  (Beneficiaries) │ │
│                                    └──────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### Core Components

| Component | Description | Status |
|---|---|---|
| **Vault Core** | Holds deposited XLM/assets, manages owner config | ✅ Scaffolded |
| **Heartbeat Monitor** | Tracks last-seen timestamp, validates owner pings | ✅ Scaffolded |
| **Timelock Engine** | Enforces 180-day grace period before release | ✅ Scaffolded |
| **Drip Stream** | Distributes assets to heirs on a schedule | 🚧 In Progress |
| **Frontend dApp** | Web UI for vault management | 🚧 In Progress |
| **Multi-sig Recovery** | Guardian-based emergency recovery | 📋 Planned |

---

## 🚀 How It Works

### 1. Owner Deposits & Configures
The owner deploys a vault, deposits assets, and registers beneficiaries with percentage allocations.

### 2. Heartbeat (Every 180 Days)
The owner sends a simple `heartbeat` transaction to prove they're alive. This resets the countdown clock.

### 3. Grace Period Triggers
If no heartbeat is detected within 180 days, the Timelock Engine activates and begins the drip release.

### 4. Drip to Heirs
Assets stream to beneficiaries over a configurable release window — preventing a single lump-sum shock and giving time for dispute resolution.

---

## 🦀 Smart Contract (Soroban)

### Heartbeat Logic

```rust
#[contractimpl]
impl StellarNexus {
    pub fn heartbeat(env: Env, owner: Address) {
        owner.require_auth();
        let current_time = env.ledger().timestamp();
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "last_seen"), &current_time);
    }

    pub fn check_and_release(env: Env) {
        let last_seen: u64 = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "last_seen"))
            .unwrap();
        let grace_period: u64 = 15_552_000; // 180 days in seconds

        if env.ledger().timestamp() > last_seen + grace_period {
            Self::trigger_drip(env);
        }
    }
}
```

### Vault Initialization

```rust
#[contractimpl]
impl StellarNexus {
    pub fn initialize(
        env: Env,
        owner: Address,
        beneficiaries: Vec<(Address, u32)>, // (address, basis_points)
    ) {
        owner.require_auth();
        assert!(
            !env.storage().instance().has(&Symbol::new(&env, "owner")),
            "already initialized"
        );

        // Validate allocations sum to 10_000 basis points (100%)
        let total: u32 = beneficiaries.iter().map(|(_, bps)| bps).sum();
        assert!(total == 10_000, "allocations must sum to 100%");

        env.storage().instance().set(&Symbol::new(&env, "owner"), &owner);
        env.storage().instance().set(&Symbol::new(&env, "beneficiaries"), &beneficiaries);
        env.storage().instance().set(
            &Symbol::new(&env, "last_seen"),
            &env.ledger().timestamp(),
        );
    }
}
```

### Drip Stream Logic

```rust
fn trigger_drip(env: Env) {
    let beneficiaries: Vec<(Address, u32)> = env
        .storage()
        .instance()
        .get(&Symbol::new(&env, "beneficiaries"))
        .unwrap();

    let vault_balance = env.current_contract_address(); // fetch actual balance

    for (heir, basis_points) in beneficiaries.iter() {
        let share = vault_balance * basis_points as i128 / 10_000;
        // Transfer share to heir — Drips integration hook
        // token_client.transfer(&env.current_contract_address(), &heir, &share);
    }
}
```

---

## 📁 Project Structure

```
StellarNexus/
├── contracts/
│   └── stellar_nexus/
│       ├── src/
│       │   ├── lib.rs          # Contract entry point
│       │   ├── heartbeat.rs    # Heartbeat & timelock logic
│       │   ├── vault.rs        # Deposit / withdrawal logic
│       │   └── drip.rs         # Beneficiary distribution
│       └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── VaultDashboard.tsx
│   │   │   ├── HeartbeatTimer.tsx
│   │   │   └── BeneficiaryManager.tsx
│   │   └── hooks/
│   │       └── useStellarNexus.ts
│   └── package.json
├── tests/
│   └── integration/
│       └── vault_test.rs
├── plan.md
└── README.md
```

---

## 🛠️ Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) + `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)
- Node.js 18+ (for frontend)

### Install & Build

```bash
# Clone the repo
git clone https://github.com/your-org/StellarNexus.git
cd StellarNexus

# Build the contract
cd contracts/stellar_nexus
cargo build --target wasm32-unknown-unknown --release

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_nexus.wasm \
  --network testnet
```

### Run Tests

```bash
cargo test
```

---

## 🗺️ Roadmap

- [x] Heartbeat mechanism
- [x] Vault initialization & beneficiary registration
- [ ] Drip stream distribution
- [ ] Frontend dApp (React + Freighter wallet)
- [ ] Multi-sig guardian recovery
- [ ] Testnet deployment & audit
- [ ] Mainnet launch

---

## 🤝 Contributing

See [plan.md](plan.md) for the contributor workflow and open issues via **The Wave Program**.

---

## 📄 License

MIT © StellarNexus Contributors
