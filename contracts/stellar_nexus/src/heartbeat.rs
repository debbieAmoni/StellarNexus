use soroban_sdk::{symbol_short, Address, Env};

use crate::drip;
use crate::vault;

pub const GRACE_PERIOD: u64 = 15_552_000; // 180 days in seconds

/// Reset the owner's heartbeat timestamp.
///
/// # Arguments
/// * `owner` - The vault owner (must authorize this call)
pub fn ping(env: &Env, owner: &Address) {
    owner.require_auth();
    env.storage()
        .instance()
        .set(&symbol_short!("last_seen"), &env.ledger().timestamp());
}

/// Trigger drip distribution if the grace period has elapsed.
/// Can be called by anyone.
pub fn check_and_release(env: &Env) {
    if vault::is_paused(env) {
        return;
    }

    let last_seen: u64 = env
        .storage()
        .instance()
        .get(&symbol_short!("last_seen"))
        .unwrap_or(0);

    if env.ledger().timestamp() > last_seen + GRACE_PERIOD {
        drip::trigger(env);
    }
}

/// Get seconds until the grace period expires; 0 if already past.
pub fn time_remaining(env: &Env) -> u64 {
    let last_seen: u64 = env
        .storage()
        .instance()
        .get(&symbol_short!("last_seen"))
        .unwrap_or(0);
    let deadline = last_seen + GRACE_PERIOD;
    let now = env.ledger().timestamp();
    if now >= deadline { 0 } else { deadline - now }
}
