use soroban_sdk::{symbol_short, Address, Env, Vec};

/// Distribute vault balance to beneficiaries by basis points.
/// TODO: replace balance tracking with a real token client transfer.
pub fn trigger(env: &Env) {
    let beneficiaries: Vec<(Address, u32)> = env
        .storage()
        .instance()
        .get(&symbol_short!("benes"))
        .expect("no beneficiaries");

    let balance: i128 = env
        .storage()
        .instance()
        .get(&symbol_short!("balance"))
        .unwrap_or(0);

    if balance == 0 {
        return;
    }

    for (heir, basis_points) in beneficiaries.iter() {
        let share = balance * basis_points as i128 / 10_000;
        // token_client.transfer(&env.current_contract_address(), &heir, &share);
        let _ = (heir, share); // placeholder until token client is wired
    }

    // Zero out balance after distribution
    env.storage()
        .instance()
        .set(&symbol_short!("balance"), &0_i128);
}
