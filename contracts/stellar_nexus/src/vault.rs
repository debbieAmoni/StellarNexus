use soroban_sdk::{contracterror, symbol_short, Address, Env, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VaultError {
    AlreadyInitialized = 1,
    InvalidAllocations = 2,
    Unauthorized = 3,
}

/// Initialize vault storage. Panics if already initialized.
pub fn initialize(env: &Env, owner: &Address, beneficiaries: Vec<(Address, u32)>) {
    owner.require_auth();

    assert!(
        !env.storage().instance().has(&symbol_short!("owner")),
        "already initialized"
    );

    let total: u32 = beneficiaries.iter().map(|(_, bps)| bps).sum();
    assert!(total == 10_000, "allocations must sum to 10_000 bps");

    env.storage().instance().set(&symbol_short!("owner"), owner);
    env.storage()
        .instance()
        .set(&symbol_short!("benes"), &beneficiaries);
    env.storage()
        .instance()
        .set(&symbol_short!("balance"), &0_i128);
    env.storage()
        .instance()
        .set(&symbol_short!("last_seen"), &env.ledger().timestamp());
}

/// Record a deposit into the vault balance.
pub fn deposit(env: &Env, owner: &Address, amount: i128) {
    owner.require_auth();
    let stored_owner: Address = env
        .storage()
        .instance()
        .get(&symbol_short!("owner"))
        .expect("not initialized");
    assert!(stored_owner == *owner, "unauthorized");

    let balance: i128 = env
        .storage()
        .instance()
        .get(&symbol_short!("balance"))
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&symbol_short!("balance"), &(balance + amount));
}

/// Read current vault balance.
pub fn balance(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("balance"))
        .unwrap_or(0)
}
