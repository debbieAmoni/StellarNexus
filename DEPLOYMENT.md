# 🚀 StellarNexus Production Deployment Guide

## Overview

This guide covers deploying StellarNexus to production on the Stellar network. The project consists of two main components:

1. **Smart Contract (Soroban)** - Handles vault logic, heartbeat timing, and drip distribution
2. **Frontend dApp** - React/TypeScript UI with Freighter wallet integration

---

## Prerequisites

### Required Software
- **Rust 1.75+** with `wasm32-unknown-unknown` target
- **Stellar CLI** v21.0+
- **Node.js 18+** and npm 9+
- **Freighter Wallet** (browser extension for testing)

### Credentials
- Stellar account with funding for:
  - Contract deployment (~100 stroops fee)
  - Test transactions (if deploying to testnet first)
- For mainnet: Funded account with XLM reserves

---

## Phase 1: Smart Contract Deployment

### 1.1 Build the Contract

```bash
cd contracts/stellar_nexus
cargo build --target wasm32-unknown-unknown --release
```

This produces a WASM binary at:
```
contracts/stellar_nexus/target/wasm32-unknown-unknown/release/stellar_nexus.wasm
```

### 1.2 Deploy to Testnet (Recommended First)

```bash
# Set your STELLAR_ACCOUNT and seed in environment
export STELLAR_ACCOUNT="your-public-key"

# Deploy to testnet
stellar contract deploy \
  --wasm contracts/stellar_nexus/target/wasm32-unknown-unknown/release/stellar_nexus.wasm \
  --network testnet
```

The command will output your **Contract ID**. Save this—you'll need it for the frontend.

### 1.3 Verify Contract on Network

```bash
# Check contract exists
stellar contract info \
  --network testnet \
  --id "CONTRACT_ID_HERE"
```

---

## Phase 2: Run Integration Tests

Before moving to mainnet, verify all contract functionality:

```bash
cd contracts/stellar_nexus
cargo test
```

Expected output: All tests pass ✅

Test coverage includes:
- Initialization and beneficiary validation
- Deposit/withdrawal logic
- Heartbeat reset mechanism
- Timelock grace period (180 days)
- Drip release distribution
- Pause/resume functionality
- Access control

---

## Phase 3: Frontend Configuration

### 3.1 Create Environment File

```bash
cp .env.example .env
```

Edit `.env`:
```env
VITE_NETWORK=testnet  # or 'mainnet'
VITE_RPC_URL=https://soroban-testnet.stellar.org
VITE_CONTRACT_ID=your-contract-id-here
VITE_APP_NAME=StellarNexus
VITE_APP_VERSION=1.0.0
```

### 3.2 Install Dependencies

```bash
cd frontend
npm install
```

### 3.3 Build Frontend

```bash
npm run build
```

This creates an optimized production build in `frontend/dist/`.

### 3.4 Test Locally (Optional)

```bash
npm run dev
```

Visit `http://localhost:5173` and connect your Freighter wallet.

---

## Phase 4: Security Audit Checklist

Before mainnet deployment, verify:

- [ ] All contract functions have proper error handling
- [ ] Access control checks are in place (owner-only functions)
- [ ] Beneficiary allocations always sum to exactly 10,000 basis points
- [ ] Token transfer failures don't leave vault in inconsistent state
- [ ] Pause/resume prevents drip release as expected
- [ ] Timestamp arithmetic prevents overflow/underflow
- [ ] No hardcoded sensitive values in code

**Recommended**: Have an external security auditor review the contract code before mainnet launch.

---

## Phase 5: Mainnet Deployment

### 5.1 Fund Your Account

Ensure your Stellar account has sufficient XLM:
```bash
# Check account balance
stellar account info --public-key "YOUR_PUBLIC_KEY"
```

### 5.2 Deploy Contract to Mainnet

```bash
# Deploy to mainnet (same as testnet, different --network flag)
stellar contract deploy \
  --wasm contracts/stellar_nexus/target/wasm32-unknown-unknown/release/stellar_nexus.wasm \
  --network mainnet
```

### 5.3 Update Frontend Configuration

Update `.env` for production:
```env
VITE_NETWORK=mainnet
VITE_RPC_URL=https://soroban-mainnet.stellar.org
VITE_CONTRACT_ID=your-mainnet-contract-id
```

Rebuild frontend:
```bash
npm run build
```

### 5.4 Deploy Frontend

Choose your hosting platform:

#### **Vercel (Recommended)**
```bash
npm install -g vercel
vercel
```

#### **Netlify**
```bash
npm install -g netlify-cli
netlify deploy --prod --dir frontend/dist
```

#### **AWS S3 + CloudFront**
```bash
aws s3 sync frontend/dist s3://your-bucket-name
```

#### **Traditional VPS (nginx)**
```bash
scp -r frontend/dist user@server:/var/www/stellarnexus/
```

Configure nginx:
```nginx
server {
    listen 443 ssl http2;
    server_name your-domain.com;
    root /var/www/stellarnexus;
    
    # SPA routing
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # Cache assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

---

## Phase 6: Post-Deployment Monitoring

### Monitor Contract Usage

```bash
# Watch testnet transactions
stellar transaction watch --network testnet --account YOUR_ACCOUNT

# View contract events (if enabled)
stellar contract log --network mainnet --id CONTRACT_ID
```

### Setup Error Alerts

Monitor your frontend for errors:
- Use Sentry, LogRocket, or similar error tracking
- Set up alerts for failed contract calls
- Monitor wallet connection issues

### Maintenance Tasks

**Weekly:**
- Check for failed transactions
- Review Freighter compatibility updates
- Monitor Stellar network status

**Monthly:**
- Audit vault balances against transactions
- Review beneficiary configuration for any inconsistencies
- Test backup/recovery procedures

---

## Troubleshooting

### Contract Deployment Fails

**Error**: `Transaction failed: insufficient fee`
- Solution: Increase fee: `--fee 10000`

**Error**: `Network not available`
- Solution: Check RPC URL and network connectivity

### Frontend Connection Issues

**Error**: "Freighter not available"
- Solution: Ensure Freighter wallet is installed and enabled

**Error**: "Contract not found on network"
- Solution: Verify CONTRACT_ID is correct and network matches

**Error**: "Transaction signature failed"
- Solution: Ensure wallet is set to the correct network (testnet/mainnet)

---

## Rollback Procedures

### Contract Rollback

Since Soroban contracts are immutable, you cannot update deployed code. If critical bugs are found:

1. Deploy a new contract with fixes
2. Update `.env` with new CONTRACT_ID
3. Redeploy frontend
4. Communicate migration steps to users

### Frontend Rollback

If frontend has issues:

```bash
# Vercel
vercel rollback

# Netlify
netlify deploy --prod --dir frontend/dist

# Manual: restore previous version from backup
```

---

## Production Support

- **Documentation**: https://developers.stellar.org
- **Stellar Developers**: https://stellar.org/developers
- **Soroban Docs**: https://soroban.stellar.org
- **Community**: Stellar Dev Discord

---

## Version History

- **1.0.0** - Initial production release
  - Core vault functionality
  - Heartbeat mechanism
  - Drip distribution
  - Pause/resume controls
  - Beneficiary management

---

## License

MIT © StellarNexus Contributors
