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

// ============ Initialization Tests ============

#[test]
fn test_initialize_succeeds_with_valid_beneficiaries() {
    let (env, client, owner, heir1, heir2) = setup();

    let result = client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 7_000u32), (heir2.clone(), 3_000u32)],
    );

    assert!(result.is_ok());
}

#[test]
fn test_initialize_fails_with_invalid_allocations() {
    let (env, client, owner, heir1, _heir2) = setup();

    let result = client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32)], // only 50%
    );

    assert!(result.is_err());
}

#[test]
fn test_initialize_fails_when_already_initialized() {
    let (env, client, owner, heir1, heir2) = setup();

    let benes = vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)];
    
    let result1 = client.initialize(&owner, &benes.clone());
    assert!(result1.is_ok());
    
    let result2 = client.initialize(&owner, &benes);
    assert!(result2.is_err());
}

#[test]
fn test_initialize_fails_with_empty_beneficiaries() {
    let (env, client, owner, _heir1, _heir2) = setup();

    let result = client.initialize(&owner, &vec![&env]);

    assert!(result.is_err());
}

// ============ Deposit Tests ============

#[test]
fn test_deposit_succeeds() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let result = client.deposit(&owner, &1_000_000_i128);
    assert!(result.is_ok());

    let balance = client.get_balance();
    assert_eq!(balance, 1_000_000_i128);
}

#[test]
fn test_deposit_fails_with_invalid_amount() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let result = client.deposit(&owner, &-100_i128);
    assert!(result.is_err());

    let result = client.deposit(&owner, &0_i128);
    assert!(result.is_err());
}

#[test]
fn test_deposit_accumulates() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    client.deposit(&owner, &500_000_i128).ok();
    client.deposit(&owner, &300_000_i128).ok();

    let balance = client.get_balance();
    assert_eq!(balance, 800_000_i128);
}

// ============ Heartbeat Tests ============

#[test]
fn test_heartbeat_resets_timer() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let initial = client.time_remaining();
    
    // Advance ledger by 90 days
    env.ledger().with_mut(|l| l.timestamp += 7_776_000);
    let before = client.time_remaining();

    client.heartbeat(&owner);
    let after = client.time_remaining();

    // After heartbeat, remaining time should increase
    assert!(after > before);
    assert!(after > initial / 2);
}

#[test]
fn test_time_remaining_initially_equals_grace_period() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let remaining = client.time_remaining();
    assert_eq!(remaining, 15_552_000u64); // 180 days in seconds
}

// ============ Drip Release Tests ============

#[test]
fn test_check_and_release_before_grace_period_does_nothing() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();
    client.deposit(&owner, &500_000_i128).ok();

    // Should not panic; grace period not elapsed
    client.check_and_release();
    
    let balance = client.get_balance();
    assert_eq!(balance, 500_000_i128); // Balance unchanged
}

#[test]
fn test_check_and_release_after_grace_period_triggers_drip() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 10_000u32)],
    ).ok();
    client.deposit(&owner, &500_000_i128).ok();

    // Advance past 180 days
    env.ledger().with_mut(|l| l.timestamp += 15_552_001);
    client.check_and_release();

    // Balance should be zeroed (drip triggered)
    let balance = client.get_balance();
    assert_eq!(balance, 0_i128);
}

// ============ Pause/Resume Tests ============

#[test]
fn test_pause_prevents_drip() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 10_000u32)],
    ).ok();
    client.deposit(&owner, &500_000_i128).ok();

    // Pause the vault
    client.pause(&owner).ok();

    // Advance past grace period
    env.ledger().with_mut(|l| l.timestamp += 15_552_001);
    client.check_and_release();

    // Balance should still be there (drip prevented by pause)
    let balance = client.get_balance();
    assert_eq!(balance, 500_000_i128);
}

#[test]
fn test_resume_allows_drip() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 10_000u32)],
    ).ok();
    client.deposit(&owner, &500_000_i128).ok();

    // Pause then resume
    client.pause(&owner).ok();
    client.resume(&owner).ok();

    // Advance past grace period
    env.ledger().with_mut(|l| l.timestamp += 15_552_001);
    client.check_and_release();

    // Balance should be zeroed (drip allowed after resume)
    let balance = client.get_balance();
    assert_eq!(balance, 0_i128);
}

// ============ Beneficiary Update Tests ============

#[test]
fn test_update_beneficiaries_succeeds() {
    let (env, client, owner, heir1, heir2) = setup();

    let heir3 = Address::generate(&env);

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let result = client.update_beneficiaries(
        &owner,
        &vec![&env, (heir1.clone(), 3_000u32), (heir2.clone(), 3_000u32), (heir3.clone(), 4_000u32)],
    );

    assert!(result.is_ok());
}

#[test]
fn test_update_beneficiaries_fails_with_invalid_allocations() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let result = client.update_beneficiaries(
        &owner,
        &vec![&env, (heir1.clone(), 6_000u32), (heir2.clone(), 3_000u32)], // 90% total
    );

    assert!(result.is_err());
}

// ============ Access Control Tests ============

#[test]
fn test_deposit_fails_if_not_owner() {
    let (env, client, owner, heir1, heir2) = setup();
    let non_owner = Address::generate(&env);

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let result = client.deposit(&non_owner, &1_000_000_i128);
    assert!(result.is_err());
}

#[test]
fn test_heartbeat_fails_if_not_owner() {
    let (env, client, owner, heir1, heir2) = setup();
    let non_owner = Address::generate(&env);

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    // This should fail because non_owner cannot auth
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.heartbeat(&non_owner);
    }));

    assert!(result.is_err());
}

// ============ Query Tests ============

#[test]
fn test_get_owner() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 5_000u32), (heir2.clone(), 5_000u32)],
    ).ok();

    let result = client.get_owner();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), owner);
}

#[test]
fn test_get_beneficiaries() {
    let (env, client, owner, heir1, heir2) = setup();

    client.initialize(
        &owner,
        &vec![&env, (heir1.clone(), 7_000u32), (heir2.clone(), 3_000u32)],
    ).ok();

    let result = client.get_beneficiaries();
    assert!(result.is_ok());
    let benes = result.unwrap();
    assert_eq!(benes.len(), 2);
}
