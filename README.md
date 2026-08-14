# 🏺 StellarNexus v1.0.0
> A "Dead Man's Switch" for the digital age. Secure your legacy with programmable inheritance on the Stellar blockchain.

[![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar-blue)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban-purple)](https://soroban.stellar.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![CI/CD](https://img.shields.io/badge/CI%2FCD-GitHub%20Actions-brightgreen)](/.github/workflows/ci-cd.yml)
[![Production Ready](https://img.shields.io/badge/Status-Production%20Ready-success)](./DEPLOYMENT.md)

---

## 🌌 What is StellarNexus?

StellarNexus is a programmable inheritance vault built on Stellar's Soroban smart contract platform. It solves one of crypto's most overlooked problems: **capital permanently locked in dead wallets**.

When an owner becomes incapacitated or passes away, their digital assets don't have to disappear. StellarNexus uses a heartbeat mechanism — a simple periodic transaction — to confirm the owner is still active. If the heartbeat stops, the vault automatically begins distributing assets to designated heirs via a drip stream.

This creates compounding **"Lindy Effect"** value: capital stays productive and circulating within the Stellar ecosystem across generations rather than being lost forever.

## ✨ Live Deployments

- **Testnet**: Available on [Stellar Test Network](https://stellar.expert/explorer/test)
- **Mainnet**: Available on [Stellar Public Network](https://stellar.expert/explorer/public)
- **Frontend**: [stellarnexus.io](https://stellarnexus.io)

---

## 🏗️ Architecture

### Smart Contract Layer
- **Vault Core**: Manages owner, balance, and beneficiary storage
- **Heartbeat Monitor**: Tracks last heartbeat and grace period countdown
- **Timelock Engine**: Enforces 180-day grace period before drip release
- **Drip Stream**: Distributes assets to beneficiaries by basis points
- **Access Control**: Owner-only functions with Soroban auth

### Frontend Layer
- **React 18**: Modern UI framework with TypeScript
- **Freighter Integration**: Stellar wallet connection and signing
- **Real-time State**: Live vault status and timer display
- **Responsive Design**: Mobile-first CSS with dark theme

---

## 🚀 Quick Start

### For Users

1. **Connect Wallet**: Install [Freighter wallet](https://www.freighter.app/)
2. **Visit App**: Go to [stellarnexus.io](https://stellarnexus.io)
3. **Create Vault**: Set beneficiaries and deposit XLM
4. **Send Heartbeat**: Confirm your vault every 180 days
5. **Relax**: Your assets are secured for your heirs

### For Developers

```bash
# Clone repository
git clone https://github.com/your-org/StellarNexus.git
cd StellarNexus

# Build smart contract
cd contracts/stellar_nexus
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Setup frontend
cd ../../frontend
npm install
npm run dev

# Visit http://localhost:5173
```

---

## 📚 Documentation

- **[Deployment Guide](./DEPLOYMENT.md)** - Complete production deployment instructions
- **[Smart Contract API](./docs/contract/api.md)** - Contract method reference
- **[User Guide](./docs/user-guide/)** - How to use StellarNexus
- **[Developer Docs](./docs/development/)** - Contributing guidelines

---

## ✨ Core Features

### Vault Management
- ✅ Create vaults with multiple beneficiaries
- ✅ Deposit and track XLM balances
- ✅ Flexible beneficiary allocations (basis points)
- ✅ Update beneficiaries at any time

### Heartbeat System
- ✅ 180-day grace period countdown
- ✅ Real-time timer display
- ✅ One-click heartbeat refresh
- ✅ Automatic drip trigger on expiration

### Safety Controls
- ✅ Pause/resume vault distribution
- ✅ Owner-only authorization
- ✅ Basis point validation (always 100%)
- ✅ Zero-balance protection

### Production Ready
- ✅ Comprehensive error handling
- ✅ Full test coverage (20+ tests)
- ✅ Security audit checklist
- ✅ CI/CD automation (GitHub Actions)
- ✅ Docker deployment support
- ✅ Responsive mobile UI

---

## 📊 Contract Functions

### Owner Operations
```rust
initialize(owner, beneficiaries)     // Create vault
deposit(owner, amount)               // Add funds
heartbeat(owner)                     // Reset countdown
pause(owner)                         // Suspend drip release
resume(owner)                        // Resume drip release
update_beneficiaries(owner, benef)  // Modify allocations
set_token_address(owner, token)     // Configure token contract
```

### Public Operations
```rust
check_and_release()          // Trigger drip if grace period elapsed
time_remaining()             // Get seconds until drip
get_balance()                // Current vault balance
get_beneficiaries()          // View allocations
get_owner()                  // Vault owner address
```

---

## 🧪 Testing

### Smart Contract Tests
```bash
cd contracts/stellar_nexus
cargo test
```

**Coverage includes:**
- ✅ Initialization & validation
- ✅ Deposit & withdrawal
- ✅ Heartbeat mechanism
- ✅ Timelock grace period
- ✅ Drip distribution
- ✅ Pause/resume functionality
- ✅ Access control
- ✅ Beneficiary management
- ✅ Error handling
- ✅ Edge cases

---

## 🔒 Security

### Contract Security
- **Owner Authentication**: All sensitive operations require owner authorization
- **Basis Point Validation**: Beneficiaries always sum to exactly 10,000 bps
- **Pause/Resume**: Emergency controls to prevent drip release
- **Error Handling**: Comprehensive error codes for debugging
- **No Reentrancy**: Soroban's structure prevents reentrancy attacks

### Frontend Security
- **Content Security Policy**: Prevents XSS attacks
- **Input Validation**: All user inputs validated before contract calls
- **Secure Connection**: HTTPS enforced on production
- **Wallet Auth**: Freighter handles key management

---

## 🛠️ Technology Stack

### Smart Contract
- **Language**: Rust
- **Framework**: Soroban SDK v21
- **Network**: Stellar (testnet + mainnet)
- **Testing**: Soroban testutils

### Frontend
- **Framework**: React 18
- **Language**: TypeScript 5
- **Wallet**: Freighter API
- **SDK**: @stellar/stellar-sdk 12+
- **Build Tool**: Vite 5
- **Styling**: CSS + CSS Modules

### DevOps
- **CI/CD**: GitHub Actions
- **Container**: Docker
- **Deployment**: Vercel / Netlify
- **Version Control**: Git

---

## 📋 Project Structure

```
StellarNexus/
├── contracts/
│   └── stellar_nexus/
│       ├── src/
│       │   ├── lib.rs              # Contract entry point
│       │   ├── vault.rs            # Deposit/withdrawal & owner management
│       │   ├── heartbeat.rs        # Heartbeat & timelock logic
│       │   └── drip.rs             # Beneficiary distribution
│       ├── Cargo.toml              # Rust dependencies
│       └── Cargo.lock
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── VaultDashboard.tsx  # Main UI
│   │   │   ├── HeartbeatTimer.tsx  # Timer display
│   │   │   └── BeneficiaryManager.tsx
│   │   ├── hooks/
│   │   │   └── useStellarNexus.ts  # Contract interface
│   │   └── types/
│   │       └── index.ts             # TypeScript definitions
│   ├── package.json
│   └── vite.config.ts
├── tests/
│   └── integration/
│       └── vault_test.rs            # 20+ integration tests
├── .github/
│   └── workflows/
│       └── ci-cd.yml                # Automated testing & deployment
├── DEPLOYMENT.md                    # Production guide
├── Dockerfile                       # Container image
├── docker-compose.yml               # Local setup
├── deploy.sh                        # Deployment script
└── README.md                        # This file
```

---

## 🚀 Deployment

### One-Command Deployment (Testnet)

```bash
./deploy.sh testnet
```

### Manual Deployment

See **[DEPLOYMENT.md](./DEPLOYMENT.md)** for complete instructions covering:
- Building the contract
- Deploying to testnet/mainnet
- Running tests
- Security audit
- Frontend deployment
- Post-deployment monitoring

---

## 🤝 Contributing

We welcome contributions! The project uses **The Wave Program** for coordinated open-source work.

### Setup Development Environment

```bash
# Clone and install dependencies
git clone https://github.com/your-org/StellarNexus.git
cd StellarNexus

# Build contract
cd contracts/stellar_nexus && cargo build

# Setup frontend
cd ../../frontend && npm install

# Run tests
cd ../contracts/stellar_nexus && cargo test
```

### Contributing Guidelines
1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit with clear messages
6. Push to your fork
7. Open a pull request

See [plan.md](plan.md) for detailed Wave Program workflow.

---

## 🐛 Reporting Issues

Found a bug? Please report it:

1. Check [existing issues](https://github.com/your-org/StellarNexus/issues)
2. [Create a new issue](https://github.com/your-org/StellarNexus/issues/new) with:
   - Clear description
   - Steps to reproduce
   - Expected vs. actual behavior
   - Environment info (OS, browser, wallet version)

---

## 📝 License

MIT © StellarNexus Contributors

See [LICENSE](./LICENSE) for full text.

---

## 🙏 Acknowledgments

- **Stellar Development Foundation** - Network infrastructure & tooling
- **Freighter Team** - Wallet integration & excellent UX
- **Community Contributors** - Feedback, testing, and improvements

---

## 📞 Support & Community

- **Website**: https://stellarnexus.io
- **Documentation**: https://stellarnexus.io/docs
- **GitHub Discussions**: https://github.com/your-org/StellarNexus/discussions
- **Discord**: https://discord.gg/your-invite
- **Email**: support@stellarnexus.io
- **Twitter**: [@StellarNexus](https://twitter.com/stellar-nexus)

---

## 🎯 Roadmap (V1.0+)

### Completed ✅
- Core vault functionality
- Heartbeat mechanism
- Drip distribution
- Frontend dApp
- Comprehensive tests
- Production deployment

### In Progress 🚧
- Advanced analytics dashboard
- Multi-sig guardian recovery
- Batch vault operations
- Advanced scheduling

### Planned 📋
- Mobile app (React Native)
- NFT integration
- DAO governance
- Enterprise features

---

**Built with ❤️ for financial freedom and legacy security**

*Secure your legacy. Let your wealth live forever.*
