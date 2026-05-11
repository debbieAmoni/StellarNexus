use soroban_sdk::{symbol_short, Address, Env};

use crate::drip;

const GRACE_PERIOD: u64 = 15_552_000; // 180 days in seconds
const KEY_LAST_SEEN: &str = "last_seen";

/// Reset the owner's heartbeat timestamp.
pub fn ping(env: &Env, owner: &Address) {
    owner.require_auth();
    env.storage()
        .instance()
        .set(&symbol_short!("last_seen"), &env.ledger().timestamp());
}

/// Trigger drip distribution if the grace period has elapsed.
pub fn check_and_release(env: &Env) {
    let last_seen: u64 = env
        .storage()
        .instance()
        .get(&symbol_short!("last_seen"))
        .expect("not initialized");

    if env.ledger().timestamp() > last_seen + GRACE_PERIOD {
        drip::trigger(env);
    }
}

/// Seconds until the grace period expires; 0 if already past.
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
