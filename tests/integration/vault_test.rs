#![cfg(test)]

use soroban_sdk::{testutils::Ledger, vec, Address, Env};

use stellar_nexus::{StellarNexus, StellarNexusClient};

fn setup() -> (Env, StellarNexusClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, StellarNexus);
    let client = StellarNexusClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let heir1 = Address::generate(&env);
    let heir2 = Address::generate(&env);

    (env, client, owner, heir1, heir2)
}

#[test]
fn test_initialize_and_deposit() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 7_000u32), (heir2.clone(), 3_000u32)],
    );
    client.deposit(&owner, &1_000_000_i128);

    // Grace period is 180 days; time_remaining should be ~15_552_000
    let remaining = client.time_remaining();
    assert!(remaining > 0);
}

#[test]
fn test_heartbeat_resets_timer() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    );

    // Advance ledger by 90 days
    env.ledger().with_mut(|l| l.timestamp += 7_776_000);
    let before = client.time_remaining();

    client.heartbeat(&owner);
    let after = client.time_remaining();

    // After heartbeat the remaining time should be greater than before
    assert!(after > before);
}

#[test]
fn test_check_and_release_before_grace_period_does_nothing() {
    let (env, client, owner, heir1, _heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 10_000u32)],
    );
    client.deposit(&owner, &500_000_i128);

    // Should not panic; grace period not elapsed
    client.check_and_release();
}

#[test]
fn test_check_and_release_after_grace_period_triggers_drip() {
    let (env, client, owner, heir1, _heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 10_000u32)],
    );
    client.deposit(&owner, &500_000_i128);

    // Advance past 180 days
    env.ledger().with_mut(|l| l.timestamp += 15_552_001);
    client.check_and_release();
    // Drip zeroes the balance; time_remaining is now 0
    assert_eq!(client.time_remaining(), 0);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_double_initialize_panics() {
    let (env, client, owner, heir1, _heir2) = setup();

    let benes = vec![&env, (heir1.clone(), 10_000u32)];
    client.initialize(&owner, &benes.clone());
    client.initialize(&owner, &benes); // should panic
}

#[test]
#[should_panic(expected = "allocations must sum to 10_000 bps")]
fn test_invalid_allocations_panics() {
    let (env, client, owner, heir1, _heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32)], // only 50%
    );
}
