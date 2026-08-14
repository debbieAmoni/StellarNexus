# StellarNexus Documentation

## Table of Contents

1. **Getting Started**
   - [Overview](./overview.md)
   - [Installation](./installation.md)
   - [Quick Start](./quickstart.md)

2. **User Guide**
   - [Creating a Vault](./user-guide/creating-vault.md)
   - [Managing Beneficiaries](./user-guide/beneficiaries.md)
   - [Heartbeat System](./user-guide/heartbeat.md)
   - [Emergency Pause](./user-guide/pause.md)

3. **Smart Contract**
   - [Architecture](./contract/architecture.md)
   - [API Reference](./contract/api.md)
   - [Security Model](./contract/security.md)
   - [Testing](./contract/testing.md)

4. **Deployment**
   - [Deployment Guide](./deployment/guide.md)
   - [Configuration](./deployment/config.md)
   - [Troubleshooting](./deployment/troubleshooting.md)

5. **Frontend**
   - [Component Architecture](./frontend/architecture.md)
   - [Hooks Reference](./frontend/hooks.md)
   - [Styling](./frontend/styling.md)

6. **Integration**
   - [Freighter Wallet](./integration/freighter.md)
   - [Stellar SDK](./integration/stellar-sdk.md)
   - [Token Contracts](./integration/tokens.md)

7. **Development**
   - [Contributing](./development/contributing.md)
   - [Code Standards](./development/standards.md)
   - [Debugging](./development/debugging.md)

---

## Quick Links

- **Website**: https://stellarnexus.io
- **GitHub**: https://github.com/your-org/StellarNexus
- **Discord**: https://discord.gg/your-invite
- **Testnet Contract**: (deployed)
- **Mainnet Contract**: (deployed)

---

## Key Concepts

### Vault
A secure container that holds digital assets and manages their eventual inheritance distribution.

### Heartbeat
A periodic signal from the vault owner to confirm they are still active. Must be sent every 180 days.

### Grace Period
The 180-day window after the last heartbeat during which drip distribution can be triggered.

### Drip Stream
The automated distribution of vault assets to beneficiaries at the configured allocation percentages.

### Basis Points (bps)
A percentage unit where 10,000 bps = 100%. For example, 5,000 bps = 50%.

---

## FAQ

**Q: How often do I need to send a heartbeat?**
A: At least once every 180 days (6 months). You can send it more frequently if desired.

**Q: Can I update my beneficiaries after vault creation?**
A: Yes, as the vault owner, you can update beneficiaries at any time.

**Q: What happens if I pause my vault?**
A: Drip release is suspended. The grace period countdown continues, but no distribution occurs until you resume.

**Q: Is this available on mainnet?**
A: Yes, StellarNexus is live on Stellar mainnet.

**Q: What networks are supported?**
A: Stellar Public Network (mainnet) and Stellar Test Network (testnet).

---

## Support

- **Documentation**: Read this guide
- **Issues**: Report bugs at https://github.com/your-org/StellarNexus/issues
- **Discussions**: Join community at https://stellar.org/developers
- **Email**: support@stellarnexus.io

---

## License

MIT © StellarNexus Contributors
