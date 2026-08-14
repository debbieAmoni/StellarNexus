use soroban_sdk::{contracterror, symbol_short, Address, Env, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VaultError {
    AlreadyInitialized = 1,
    InvalidAllocations = 2,
    Unauthorized = 3,
    NotInitialized = 4,
    InvalidAmount = 5,
    InsufficientBalance = 6,
    InvalidBeneficiary = 7,
}

/// Initialize vault storage with owner and beneficiaries.
///
/// # Arguments
/// * `owner` - The vault owner address (must authorize this call)
/// * `beneficiaries` - Vec of (address, basis_points) where sum must equal 10_000
///
/// # Errors
/// * `AlreadyInitialized` - Vault already initialized
/// * `InvalidAllocations` - Beneficiary allocations don't sum to 10_000 bps
pub fn initialize(env: &Env, owner: &Address, beneficiaries: Vec<(Address, u32)>) -> Result<(), VaultError> {
    owner.require_auth();

    if env.storage().instance().has(&symbol_short!("owner")) {
        return Err(VaultError::AlreadyInitialized);
    }

    let total: u32 = beneficiaries.iter().map(|(_, bps)| bps).sum();
    if total != 10_000 {
        return Err(VaultError::InvalidAllocations);
    }

    if beneficiaries.len() == 0 {
        return Err(VaultError::InvalidBeneficiary);
    }

    env.storage().instance().set(&symbol_short!("owner"), owner);
    env.storage()
        .instance()
        .set(&symbol_short!("benes"), &beneficiaries);
    env.storage()
        .instance()
        .set(&symbol_short!("balance"), &0_i128);
    env.storage()
        .instance()
        .set(&symbol_short!("paused"), &false);
    env.storage()
        .instance()
        .set(&symbol_short!("last_seen"), &env.ledger().timestamp());
    
    Ok(())
}

/// Record a deposit into the vault balance.
///
/// # Arguments
/// * `owner` - The vault owner address (must authorize this call)
/// * `amount` - Amount to deposit (must be positive)
///
/// # Errors
/// * `NotInitialized` - Vault not initialized
/// * `Unauthorized` - Caller is not the owner
/// * `InvalidAmount` - Amount is not positive
pub fn deposit(env: &Env, owner: &Address, amount: i128) -> Result<(), VaultError> {
    owner.require_auth();

    if amount <= 0 {
        return Err(VaultError::InvalidAmount);
    }

    let stored_owner: Address = env
        .storage()
        .instance()
        .get(&symbol_short!("owner"))
        .ok_or(VaultError::NotInitialized)?;
    
    if stored_owner != *owner {
        return Err(VaultError::Unauthorized);
    }

    let balance: i128 = env
        .storage()
        .instance()
        .get(&symbol_short!("balance"))
        .unwrap_or(0);
    
    let new_balance = balance.checked_add(amount).ok_or(VaultError::InvalidAmount)?;
    env.storage()
        .instance()
        .set(&symbol_short!("balance"), &new_balance);
    
    Ok(())
}

/// Read current vault balance.
pub fn balance(env: &Env) -> i128 {
    env.storage()
        .instance()
        .get(&symbol_short!("balance"))
        .unwrap_or(0)
}

/// Get the vault owner.
pub fn get_owner(env: &Env) -> Result<Address, VaultError> {
    env.storage()
        .instance()
        .get(&symbol_short!("owner"))
        .ok_or(VaultError::NotInitialized)
}

/// Get all beneficiaries with their allocations.
pub fn get_beneficiaries(env: &Env) -> Result<Vec<(Address, u32)>, VaultError> {
    env.storage()
        .instance()
        .get(&symbol_short!("benes"))
        .ok_or(VaultError::NotInitialized)
}

/// Check if vault is paused.
pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&symbol_short!("paused"))
        .unwrap_or(false)
}

/// Pause vault (owner only).
pub fn pause(env: &Env, owner: &Address) -> Result<(), VaultError> {
    owner.require_auth();
    let stored_owner = get_owner(env)?;
    
    if stored_owner != *owner {
        return Err(VaultError::Unauthorized);
    }
    
    env.storage().instance().set(&symbol_short!("paused"), &true);
    Ok(())
}

/// Resume vault (owner only).
pub fn resume(env: &Env, owner: &Address) -> Result<(), VaultError> {
    owner.require_auth();
    let stored_owner = get_owner(env)?;
    
    if stored_owner != *owner {
        return Err(VaultError::Unauthorized);
    }
    
    env.storage().instance().set(&symbol_short!("paused"), &false);
    Ok(())
}

/// Update beneficiaries (owner only).
pub fn update_beneficiaries(env: &Env, owner: &Address, beneficiaries: Vec<(Address, u32)>) -> Result<(), VaultError> {
    owner.require_auth();
    let stored_owner = get_owner(env)?;
    
    if stored_owner != *owner {
        return Err(VaultError::Unauthorized);
    }

    let total: u32 = beneficiaries.iter().map(|(_, bps)| bps).sum();
    if total != 10_000 {
        return Err(VaultError::InvalidAllocations);
    }

    if beneficiaries.len() == 0 {
        return Err(VaultError::InvalidBeneficiary);
    }

    env.storage()
        .instance()
        .set(&symbol_short!("benes"), &beneficiaries);
    
    Ok(())
}
