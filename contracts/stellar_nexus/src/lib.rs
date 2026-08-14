#![no_std]

mod heartbeat;
mod vault;
mod drip;

use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

pub use vault::VaultError;

#[contract]
pub struct StellarNexus;

#[contractimpl]
impl StellarNexus {
    /// Initialize the vault with an owner and beneficiary allocations.
    /// 
    /// # Arguments
    /// * `owner` - The vault owner address (must authorize)
    /// * `beneficiaries` - Vec of (address, basis_points) where sum must equal 10_000
    /// 
    /// # Returns
    /// Ok(()) on success, Err(VaultError) on failure
    pub fn initialize(
        env: Env,
        owner: Address,
        beneficiaries: Vec<(Address, u32)>,
    ) -> Result<(), VaultError> {
        vault::initialize(&env, &owner, beneficiaries)
    }

    /// Owner deposits XLM-equivalent token amount into the vault.
    /// 
    /// # Arguments
    /// * `owner` - The vault owner address (must authorize)
    /// * `amount` - Amount to deposit (must be positive)
    pub fn deposit(env: Env, owner: Address, amount: i128) -> Result<(), VaultError> {
        vault::deposit(&env, &owner, amount)
    }

    /// Owner pings the contract to reset the 180-day countdown.
    /// 
    /// # Arguments
    /// * `owner` - The vault owner address (must authorize)
    pub fn heartbeat(env: Env, owner: Address) {
        heartbeat::ping(&env, &owner);
    }

    /// Anyone may call this. Triggers drip if grace period has elapsed.
    pub fn check_and_release(env: Env) {
        heartbeat::check_and_release(&env);
    }

    /// Returns seconds remaining before drip activates (0 if already triggered).
    pub fn time_remaining(env: Env) -> u64 {
        heartbeat::time_remaining(&env)
    }

    /// Get the current vault balance.
    pub fn get_balance(env: Env) -> i128 {
        vault::balance(&env)
    }

    /// Get the vault owner.
    pub fn get_owner(env: Env) -> Result<Address, VaultError> {
        vault::get_owner(&env)
    }

    /// Get all beneficiaries with their allocations.
    pub fn get_beneficiaries(env: Env) -> Result<Vec<(Address, u32)>, VaultError> {
        vault::get_beneficiaries(&env)
    }

    /// Pause the vault (owner only). Prevents drip release while paused.
    pub fn pause(env: Env, owner: Address) -> Result<(), VaultError> {
        vault::pause(&env, &owner)
    }

    /// Resume the vault (owner only).
    pub fn resume(env: Env, owner: Address) -> Result<(), VaultError> {
        vault::resume(&env, &owner)
    }

    /// Update beneficiaries (owner only).
    pub fn update_beneficiaries(
        env: Env,
        owner: Address,
        beneficiaries: Vec<(Address, u32)>,
    ) -> Result<(), VaultError> {
        vault::update_beneficiaries(&env, &owner, beneficiaries)
    }

    /// Set the token contract address for transfers.
    pub fn set_token_address(env: Env, owner: Address, token_address: Address) -> Result<(), VaultError> {
        drip::set_token(&env, &owner, &token_address)
    }
}
