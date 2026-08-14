#!/bin/bash
# StellarNexus Frontend - Vercel Deployment Quick Start
# This script automates the Vercel deployment process

set -e

echo "🚀 StellarNexus Frontend - Vercel Deployment"
echo "=============================================="
echo ""

# Check if Vercel CLI is installed
if ! command -v vercel &> /dev/null; then
    echo "📦 Installing Vercel CLI..."
    npm install -g vercel
fi

# Check if user is logged in
echo "🔐 Checking Vercel authentication..."
if ! vercel whoami &> /dev/null; then
    echo "📱 Please log in to Vercel (browser will open)..."
    vercel login
fi

# Move to frontend directory
cd frontend

# Install dependencies
echo ""
echo "📥 Installing dependencies..."
npm install

# Build locally first
echo ""
echo "🔨 Building frontend locally..."
npm run build

if [ ! -d "dist" ]; then
    echo "❌ Build failed: dist directory not created"
    exit 1
fi

echo "✅ Build successful"

# Deploy to Vercel
echo ""
echo "🚀 Deploying to Vercel..."
vercel --prod

echo ""
echo "✅ Deployment complete!"
echo ""
echo "📝 Next steps:"
echo "1. Go to https://vercel.com/dashboard"
echo "2. Select your StellarNexus project"
echo "3. Click 'Settings' → 'Environment Variables'"
echo "4. Add the following variables:"
echo "   - VITE_NETWORK: testnet (or mainnet)"
echo "   - VITE_RPC_URL: https://soroban-testnet.stellar.org"
echo "   - VITE_CONTRACT_ID: <your-contract-id>"
echo "5. Redeploy or wait for next commit"
echo ""
echo "🎉 Your app will be live at: https://stellarnexus.vercel.app"
