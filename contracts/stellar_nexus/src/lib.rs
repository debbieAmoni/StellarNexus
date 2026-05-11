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
    /// `beneficiaries` is a list of (address, basis_points) where basis_points sum to 10_000.
    pub fn initialize(env: Env, owner: Address, beneficiaries: Vec<(Address, u32)>) {
        vault::initialize(&env, &owner, beneficiaries);
    }

    /// Owner deposits XLM-equivalent token amount into the vault.
    pub fn deposit(env: Env, owner: Address, amount: i128) {
        vault::deposit(&env, &owner, amount);
    }

    /// Owner pings the contract to reset the 180-day countdown.
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
}
