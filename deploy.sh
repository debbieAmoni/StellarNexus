#!/bin/bash
# StellarNexus Deployment Script
# Deploys the Soroban smart contract to Stellar network

set -e

# Configuration
NETWORK="${1:-testnet}"
RPC_URL=""
NETWORK_PASSPHRASE=""

case $NETWORK in
  testnet)
    RPC_URL="https://soroban-testnet.stellar.org"
    NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
    ;;
  mainnet)
    RPC_URL="https://soroban-mainnet.stellar.org"
    NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
    ;;
  *)
    echo "Invalid network: $NETWORK. Use 'testnet' or 'mainnet'."
    exit 1
    ;;
esac

echo "🚀 StellarNexus Deployment"
echo "Network: $NETWORK"
echo "RPC: $RPC_URL"
echo ""

# Build contract
echo "📦 Building contract..."
cd contracts/stellar_nexus
cargo build --target wasm32-unknown-unknown --release
cd ../..

WASM_FILE="contracts/stellar_nexus/target/wasm32-unknown-unknown/release/stellar_nexus.wasm"

if [ ! -f "$WASM_FILE" ]; then
  echo "❌ Failed: WASM file not found at $WASM_FILE"
  exit 1
fi

echo "✅ Build successful"
echo ""

# Deploy contract
echo "📡 Deploying to $NETWORK..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_FILE" \
  --network "$NETWORK" 2>&1 | grep -oP 'Contract ID: \K[A-Z0-9]+' || true)

if [ -z "$CONTRACT_ID" ]; then
  echo "❌ Deployment failed. Please check your credentials and network."
  exit 1
fi

echo "✅ Deployment successful!"
echo "Contract ID: $CONTRACT_ID"
echo ""

# Save contract ID
echo "Saving contract ID to deployment record..."
cat > "deployments/$NETWORK-$(date +%s).json" <<EOF
{
  "network": "$NETWORK",
  "contractId": "$CONTRACT_ID",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "rpcUrl": "$RPC_URL"
}
EOF

# Update .env
echo "Updating .env file..."
if [ -f ".env" ]; then
  sed -i.bak "s/VITE_CONTRACT_ID=.*/VITE_CONTRACT_ID=$CONTRACT_ID/" .env
  rm -f .env.bak
else
  cp .env.example .env
  sed -i.bak "s/VITE_CONTRACT_ID=.*/VITE_CONTRACT_ID=$CONTRACT_ID/" .env
  rm -f .env.bak
fi

echo "✅ Configuration updated"
echo ""
echo "📝 Next steps:"
echo "1. Update frontend/.env with VITE_CONTRACT_ID=$CONTRACT_ID"
echo "2. Run 'npm install && npm run build' in the frontend directory"
echo "3. Deploy to your hosting provider"
