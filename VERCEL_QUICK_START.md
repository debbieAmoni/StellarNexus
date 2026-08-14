# 🚀 StellarNexus - Vercel Deployment Quick Start

**Status**: Ready for Vercel deployment  
**Time to Deploy**: ~5 minutes  
**Cost**: Free tier available

---

## 🎯 Option 1: Automated Deployment (Easiest)

### One Command Deploy

```bash
./deploy-vercel.sh
```

This script will:
1. ✅ Check Vercel CLI installation (install if needed)
2. ✅ Authenticate with Vercel (opens browser)
3. ✅ Install frontend dependencies
4. ✅ Build the frontend
5. ✅ Deploy to Vercel production
6. ✅ Show you the live URL

**That's it!** Your app is deployed. 🎉

---

## 🎯 Option 2: Manual Vercel CLI Deployment

### Step 1: Install Vercel CLI

```bash
npm install -g vercel
```

### Step 2: Authenticate

```bash
vercel login
```
Browser opens → Authenticate with GitHub

### Step 3: Deploy

```bash
cd frontend
npm install
npm run build
vercel --prod
```

**Live URL**: https://stellarnexus.vercel.app (or custom domain)

---

## 🎯 Option 3: GitHub Dashboard (No CLI Required)

### Step 1: Push to GitHub

```bash
git push origin main
```

### Step 2: Connect to Vercel

1. Go to https://vercel.com
2. Click "New Project"
3. Select GitHub repository
4. Click "Import"

### Step 3: Configure

| Setting | Value |
|---------|-------|
| Framework | Vite (auto-detected) |
| Root Directory | `./frontend` |
| Build Command | `npm run build` |
| Output Directory | `dist` |

### Step 4: Add Environment Variables

Click "Environment Variables" and add:

```env
VITE_NETWORK=testnet
VITE_RPC_URL=https://soroban-testnet.stellar.org
VITE_CONTRACT_ID=<contract-id-from-deploy.sh>
VITE_APP_NAME=StellarNexus
VITE_APP_VERSION=1.0.0
```

### Step 5: Deploy

Click "Deploy" → Wait for completion → ✅ Live!

---

## ⚙️ Required Environment Variables

Add these to Vercel (Settings → Environment Variables):

```
VITE_NETWORK=testnet              # testnet or mainnet
VITE_RPC_URL=https://soroban-testnet.stellar.org  # RPC endpoint
VITE_CONTRACT_ID=CXXXXXX...       # Contract ID from deploy.sh
VITE_APP_NAME=StellarNexus        # App name
VITE_APP_VERSION=1.0.0            # Version
```

**Get Contract ID:**
```bash
./deploy.sh testnet
# Output: Contract ID: CXXXXX...
```

---

## ✅ Verification Checklist

After deployment, verify:

- [ ] Frontend builds successfully
- [ ] Vercel URL is accessible
- [ ] Page loads without errors
- [ ] Freighter wallet connects
- [ ] Can view vault state
- [ ] Contract interaction works
- [ ] Mobile responsive works
- [ ] Dark theme displays correctly

---

## 🔄 Redeploy After Updates

### Auto-Redeploy (Recommended)

Just commit and push:
```bash
git add .
git commit -m "Update feature"
git push origin main
```

Vercel automatically redeploys! 🚀

### Manual Redeploy

```bash
vercel --prod
```

---

## 🌍 Custom Domain

### Add Your Domain

1. Vercel Dashboard → Your Project
2. Settings → Domains
3. Add your domain (e.g., `stellarnexus.io`)
4. Update DNS records at registrar:
   ```
   CNAME  stellarnexus.io  cname.vercel.com
   ```

DNS propagation: 5-48 hours

---

## 🔧 Troubleshooting

### "npm: command not found"
- Install Node.js: https://nodejs.org (v18+ required)
- Verify: `node --version`

### "vercel: command not found"
- Install Vercel CLI: `npm install -g vercel`
- Verify: `vercel --version`

### Build Fails
```bash
cd frontend
rm -rf node_modules package-lock.json
npm install
npm run build
```

### Contract Not Found
- Verify `VITE_CONTRACT_ID` in environment variables
- Check contract was deployed: `./deploy.sh testnet`
- Update environment variable with correct ID
- Redeploy: `vercel --prod`

### Wallet Not Connecting
- Install [Freighter Wallet](https://www.freighter.app/)
- Check wallet is on correct network (testnet/mainnet)
- Verify `VITE_NETWORK` matches wallet network
- Refresh page

### Blank Page
1. Open browser DevTools (F12)
2. Check Console tab for errors
3. Check Network tab for failed requests
4. Verify contract and RPC endpoints are correct

---

## 📊 Deployment Status Dashboard

View your deployment:

```bash
vercel list              # Show all deployments
vercel logs              # Show live logs
vercel env list          # Show environment variables
vercel inspect           # Inspect project settings
```

---

## 🎯 After Deployment

### Share Your URL
```
https://stellarnexus.vercel.app
```

### Monitor Performance
- Vercel Dashboard → Analytics
- View page load times, Core Web Vitals
- Monitor error rates

### Update Contract (When Ready)

For mainnet deployment:
```bash
./deploy.sh mainnet
# Get mainnet contract ID
# Update VITE_CONTRACT_ID in Vercel
# Update VITE_NETWORK to mainnet
# Update VITE_RPC_URL
# Vercel auto-redeploys
```

---

## 💡 Tips

- **Vercel Free Tier**: Includes 100GB bandwidth/month (plenty!)
- **Auto-Scaling**: Vercel handles traffic spikes automatically
- **Zero Config**: Most settings detected automatically
- **GitHub Integration**: Auto-deploy on every push
- **Preview URLs**: Each PR gets a preview deployment
- **Rollback**: Revert to previous version with one click

---

## 🚀 Your Deployment is Ready!

Everything is configured and ready to deploy. Choose one of three methods above and you'll be live in minutes!

### Quick Commands Reference

```bash
# Automated
./deploy-vercel.sh

# Manual CLI
cd frontend && npm install && npm run build && vercel --prod

# Check status
vercel list && vercel env list

# Redeploy
vercel --prod

# View logs
vercel logs https://stellarnexus.vercel.app
```

---

**Happy deploying! 🎉**

Questions? See [VERCEL_DEPLOYMENT.md](./VERCEL_DEPLOYMENT.md) for detailed guide.
