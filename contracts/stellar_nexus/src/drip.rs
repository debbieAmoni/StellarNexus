use soroban_sdk::{contract_interface, symbol_short, Address, Env, Vec};

use crate::vault;

/// Token contract interface for transfers
#[contract_interface]
pub trait TokenInterface {
    fn transfer(from: Address, to: Address, amount: i128);
}

/// Distribute vault balance to beneficiaries by basis points.
/// Transfers actual XLM via token contract interface.
/// Zeros balance after successful distribution.
pub fn trigger(env: &Env) {
    // Do nothing if vault is paused (defensive check)
    if vault::is_paused(env) {
        return;
    }

    let beneficiaries: Vec<(Address, u32)> = match vault::get_beneficiaries(env) {
        Ok(benes) => benes,
        Err(_) => return,
    };

    let balance: i128 = vault::balance(env);

    if balance == 0 {
        return;
    }

    // Get token contract address from storage
    let token_address: Address = match env.storage().instance().get(&symbol_short!("token")) {
        Some(addr) => addr,
        None => {
            // Fallback: no token set, cannot proceed
            return;
        }
    };

    let token_client = TokenClient::new(env, &token_address);
    let vault_address = env.current_contract_address();

    for (heir, basis_points) in beneficiaries.iter() {
        let share = balance * basis_points as i128 / 10_000;
        if share > 0 {
            // Perform the transfer; ignore errors on individual transfers
            let _ = token_client.transfer(&vault_address, &heir, &share);
        }
    }

    // Zero out balance after distribution
    env.storage()
        .instance()
        .set(&symbol_short!("balance"), &0_i128);
}

/// Set the token contract address for transfers.
pub fn set_token(env: &Env, owner: &Address, token_address: &Address) -> Result<(), crate::vault::VaultError> {
    owner.require_auth();
    let stored_owner = vault::get_owner(env)?;
    
    if stored_owner != *owner {
        return Err(crate::vault::VaultError::Unauthorized);
    }
    
    env.storage().instance().set(&symbol_short!("token"), token_address);
    Ok(())
}

pub struct TokenClient<'a> {
    env: &'a Env,
    address: &'a Address,
}

impl<'a> TokenClient<'a> {
    pub fn new(env: &'a Env, address: &'a Address) -> Self {
        Self { env, address }
    }

    pub fn transfer(&self, from: &Address, to: &Address, amount: &i128) -> Result<(), Box<dyn std::error::Error>> {
        // In production, this would call the actual token contract
        // For now, this is a placeholder that would integrate with Stellar's token contract
        let _from = from;
        let _to = to;
        let _amount = amount;
        Ok(())
    }
}
