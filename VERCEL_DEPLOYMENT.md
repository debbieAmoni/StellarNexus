# 🚀 StellarNexus Frontend - Vercel Deployment Guide

## Quick Start (Easiest Method)

### Option 1: Deploy via Vercel Dashboard (No CLI Required)

1. **Push to GitHub** (if not already there)
   ```bash
   git init
   git add .
   git commit -m "Initial StellarNexus v1.0.0"
   git push -u origin main
   ```

2. **Go to Vercel**: https://vercel.com
   - Sign up or log in with GitHub
   - Click "New Project"
   - Select your `StellarNexus` repository
   - Click "Import"

3. **Configure Project**
   - **Framework**: Vite (auto-detected)
   - **Root Directory**: `./frontend`
   - **Build Command**: `npm run build`
   - **Output Directory**: `dist`

4. **Add Environment Variables**
   - Click "Environment Variables"
   - Add the following:
     ```
     VITE_NETWORK=testnet
     VITE_RPC_URL=https://soroban-testnet.stellar.org
     VITE_CONTRACT_ID=<your-contract-id-from-deploy.sh>
     VITE_APP_NAME=StellarNexus
     VITE_APP_VERSION=1.0.0
     ```

5. **Deploy**
   - Click "Deploy"
   - Wait for build to complete
   - Your app is live! 🎉

---

## Option 2: Deploy via Vercel CLI

### Step 1: Install Vercel CLI

```bash
npm install -g vercel
```

### Step 2: Configure Environment (First Time)

Create `.env.production.local` in the `frontend` directory:

```env
VITE_NETWORK=testnet
VITE_RPC_URL=https://soroban-testnet.stellar.org
VITE_CONTRACT_ID=<your-contract-id>
VITE_APP_NAME=StellarNexus
VITE_APP_VERSION=1.0.0
```

### Step 3: Login to Vercel

```bash
vercel login
```

Follow the prompts to authenticate with GitHub.

### Step 4: Deploy Frontend

```bash
cd frontend
vercel
```

**First deployment prompt:**
- Project name: `StellarNexus`
- Root directory: `./frontend` (or accept default)
- Build command: `npm run build`
- Output directory: `dist`

**Follow-up:**
- Link to existing project? → `No` (first time)
- After completion, your URL appears:
  ```
  https://stellarnexus.vercel.app
  ```

### Step 5: Update Environment Variables

```bash
vercel env add VITE_CONTRACT_ID
# Enter: <your-contract-id>
# Select: production

vercel env add VITE_NETWORK
# Enter: testnet
# Select: production
```

Or edit in Vercel dashboard:
1. Go to [vercel.com/dashboard](https://vercel.com/dashboard)
2. Select your project
3. Click "Settings" → "Environment Variables"
4. Add all variables

---

## Configuration Files

### vercel.json (Already Created)
```json
{
  "buildCommand": "npm run build",
  "outputDirectory": "dist",
  "framework": "vite",
  "env": {
    "VITE_NETWORK": "testnet",
    "VITE_RPC_URL": "https://soroban-testnet.stellar.org"
  },
  "rewrites": [
    {
      "source": "/(.*)",
      "destination": "/index.html"
    }
  ]
}
```

### .vercelignore (Already Created)
Excludes unnecessary files from deployment.

---

## Production Configuration (After Contract Deployment)

### Update for Mainnet

Once you've deployed the contract to mainnet:

1. **Get Mainnet Contract ID**
   ```bash
   ./deploy.sh mainnet
   # Contract ID: CDxxxxxxx...
   ```

2. **Update Environment Variables in Vercel Dashboard**
   - Go to Settings → Environment Variables
   - Update:
     ```
     VITE_NETWORK=mainnet
     VITE_RPC_URL=https://soroban-mainnet.stellar.org
     VITE_CONTRACT_ID=<mainnet-contract-id>
     ```

3. **Redeploy**
   ```bash
   vercel --prod
   ```

---

## Deployment Checklist

Before deploying:

- [ ] Smart contract deployed (testnet or mainnet)
- [ ] Contract ID obtained from `deploy.sh`
- [ ] Frontend dependencies installed (`npm install`)
- [ ] Build succeeds locally (`npm run build`)
- [ ] Environment variables configured in Vercel
- [ ] GitHub repository is public (recommended)
- [ ] Git commits are up to date

---

## After Deployment

### Verify Deployment

1. **Check Build Status**
   ```bash
   vercel list
   ```

2. **View Logs**
   ```bash
   vercel logs https://stellarnexus.vercel.app
   ```

3. **Test the App**
   - Visit: https://your-deployment.vercel.app
   - Connect Freighter wallet
   - Create a test vault
   - Verify contract interaction

### Monitoring

Vercel provides:
- **Performance Analytics** - Page load times, Core Web Vitals
- **Real-time Logs** - Errors, requests, deployments
- **Automatic Backups** - Previous deployments accessible

View in Vercel Dashboard:
1. Select your project
2. Click "Analytics" or "Logs"
3. Monitor in real-time

---

## Troubleshooting

### Build Fails

**Error**: `Cannot find module '@stellar/stellar-sdk'`
- Solution: Ensure dependencies installed
  ```bash
  cd frontend
  npm install
  npm run build
  ```

### App Loads Blank Page

**Error**: CORS or Contract Not Found
- Check browser console (F12 → Console)
- Verify `VITE_CONTRACT_ID` is set correctly
- Check `VITE_RPC_URL` is accessible

### Wallet Connection Issues

**Error**: "Freighter not found" or "Not connected"
- Install [Freighter Wallet](https://www.freighter.app/)
- Refresh page
- Check wallet is on same network (testnet/mainnet)

### Environment Variables Not Working

**Error**: Variables are undefined
- Verify in Vercel Dashboard
- Restart deployment
- Check `.env.production.local` exists
- Clear browser cache

---

## Custom Domain

### Connect Your Domain

1. Go to Vercel Dashboard
2. Select your project
3. Click "Settings" → "Domains"
4. Enter your domain
5. Follow DNS instructions
6. Update DNS records at your registrar

Example DNS records:
```
CNAME  stellarnexus.yourdomain.com  cname.vercel.com
```

---

## Auto-Deployment from GitHub

Once connected, Vercel auto-deploys on:
- Push to `main` branch → Production
- Push to `develop` branch → Preview

Configure in Vercel Settings:
1. Go to "Settings" → "Git"
2. Set "Production Branch" to `main`
3. Enable "Automatic Deployments"

---

## Performance Optimization

Vercel includes automatic:
- ✅ Image optimization
- ✅ Code splitting
- ✅ Minification
- ✅ Compression (gzip/brotli)
- ✅ CDN caching
- ✅ Edge functions (if needed)

No additional configuration needed!

---

## Pricing

**Vercel Free Tier Includes:**
- Unlimited deployments
- 100 GB bandwidth/month
- Serverless functions
- Perfect for StellarNexus frontend

No payment required to get started!

---

## Support

- **Vercel Docs**: https://vercel.com/docs
- **StellarNexus Docs**: ./DEPLOYMENT.md
- **Discord**: https://discord.gg/your-invite

---

## Next Steps

1. ✅ Deploy frontend to Vercel
2. Deploy contract to testnet/mainnet
3. Update CONTRACT_ID in environment variables
4. Redeploy (Vercel auto-rebuilds)
5. Share your URL with the community!

---

**Deployed with ❤️ on Vercel**
