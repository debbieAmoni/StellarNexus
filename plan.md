# 🌊 The Wave Program — Contributor Plan

StellarNexus is built in public. The Wave Program is how we coordinate open-source contributions: maintainers post scoped issues, contributors pick them up during sprint cycles, and working code ships every two weeks.

---

## How It Works

1. **Maintainers open scoped issues** tagged by type and difficulty before each sprint starts.
2. **Contributors claim an issue** by commenting — one issue per contributor per sprint.
3. **A two-week sprint** runs. Contributors submit a PR before the deadline.
4. **Maintainers review** within 48 hours of submission. Merged PRs earn Wave credits.
5. **Repeat.** Credits accumulate toward recognition, early access, and future governance weight.

---

## Types of Work

### 🐛 Bug Fixes
The most accessible entry point. Issues are labeled `bug` and include reproduction steps, expected vs. actual behavior, and the affected file(s). Examples:
- Heartbeat timestamp not resetting correctly after re-initialization
- Basis-point validation allowing allocations over 100%
- Token transfer failing silently when vault balance is zero

Good for: first-time contributors, anyone learning Soroban.

---

### ✨ New Features
Scoped feature work tied to the roadmap. Each issue includes an acceptance criteria checklist so there's no ambiguity about what "done" means. Examples:
- Implement `trigger_drip` to distribute vault balance to beneficiaries by basis points
- Add `update_beneficiaries` function with owner-auth guard
- Build `pause_vault` / `resume_vault` emergency controls
- Multi-sig guardian recovery flow (advanced)

Good for: contributors comfortable with Rust and Soroban's storage/auth model.

---

### 📖 Documentation
Docs are first-class work here. Issues are labeled `docs` and are never treated as filler. Examples:
- Write inline `///` doc comments for all public contract functions
- Add a "How Drips Work" explainer section to README
- Document the testnet deployment flow end-to-end
- Translate README to Spanish or Portuguese

Good for: technical writers, developers who want low-friction first contributions.

---

### 🧪 Testing
The contract needs coverage before mainnet. Issues are labeled `testing` and specify which functions or edge cases need tests. Examples:
- Integration test: heartbeat resets countdown correctly
- Edge case: `initialize` called twice should panic
- Fuzz test: random basis-point allocations always sum-check correctly
- Frontend: unit tests for `HeartbeatTimer` component countdown logic

Good for: contributors who enjoy finding edge cases and breaking things safely.

---

### 🎨 Frontend / UX
React + Freighter wallet work. Issues are labeled `frontend` and include Figma references or wireframe descriptions where relevant. Examples:
- Build `BeneficiaryManager` component (add/remove/rebalance heirs)
- Connect `HeartbeatTimer` to live contract state via Stellar SDK
- Add wallet connection flow with Freighter
- Mobile-responsive layout for `VaultDashboard`

Good for: frontend developers new to Web3 who want a real Stellar integration to learn from.

---

## Sprint Schedule

| Phase | Duration | Activity |
|---|---|---|
| Issue Triage | Days 1–2 | Maintainers open and label sprint issues |
| Claiming | Days 2–3 | Contributors comment to claim |
| Build | Days 3–12 | Active development |
| PR Deadline | Day 13 | All PRs submitted |
| Review & Merge | Days 13–14 | Maintainer review window |

---

## Contribution Standards

- PRs must include tests for any logic change.
- All Soroban functions must have `///` doc comments.
- Follow existing code style — no new dependencies without maintainer approval.
- One PR per issue. Keep scope tight.

---

## Get Involved

Browse open issues → `github.com/your-org/StellarNexus/issues`

Join the discussion → tag your PR with `wave-program` and link the issue it closes.
