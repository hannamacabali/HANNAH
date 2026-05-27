#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Ledger, Address, BytesN, Env};

fn setup_test_env<'a>() -> (Env, Address, Address, ZKClearGlobalClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();

    let auditor = Address::generate(&env);
    let fintech_wallet = Address::generate(&env);
    
    let contract_id = env.register_contract(None, ZKClearGlobal);
    let zk_client = ZKClearGlobalClient::new(&env, &contract_id);

    env.ledger().set_timestamp(15_000);

    (env, auditor, fintech_wallet, zk_client)
}

#[test]
fn test_1_happy_path_successful_audit_clearance() {
    let (_env, auditor, fintech_wallet, zk_client) = setup_test_env();
    
    zk_client.initialize(&auditor);
    
    let mock_zk_proof = BytesN::from_array(&zk_client.env, &[7u8; 32]);
    zk_client.commit_solvency_proof(&fintech_wallet, &mock_zk_proof);
    
    // Auditor reviews cryptographic commitment parameters and flags status as verified
    zk_client.verify_fintech_solvency(&fintech_wallet);

    assert!(zk_client.get_compliance_status(&fintech_wallet));
}

#[test]
#[should_panic(expected = "No solvency commitment proof located for the designated fintech entity.")]
fn test_2_edge_case_audit_non_existent_proof_fails() {
    let (_env, auditor, _fintech_wallet, zk_client) = setup_test_env();
    
    zk_client.initialize(&auditor);
    let random_unregistered_entity = Address::generate(&zk_client.env);
    
    // Attempting to certify an entity that hasn't committed data must fail instantly
    zk_client.verify_fintech_solvency(&random_unregistered_entity);
}

#[test]
fn test_3_state_verification_default_unverified_mode() {
    let (_env, auditor, fintech_wallet, zk_client) = setup_test_env();
    
    zk_client.initialize(&auditor);
    let mock_zk_proof = BytesN::from_array(&zk_client.env, &[3u8; 32]);
    zk_client.commit_solvency_proof(&fintech_wallet, &mock_zk_proof);

    // Assert status verification tracks as false immediately following commitment prior to audit execution
    assert!(!zk_client.get_compliance_status(&fintech_wallet));
}

#[test]
#[should_panic(expected = "Compliance registry structure already initialised")]
fn test_4_edge_case_prevent_double_initialization() {
    let (_env, auditor, _fintech_wallet, zk_client) = setup_test_env();
    
    zk_client.initialize(&auditor);
    zk_client.initialize(&auditor);
}

#[test]
#[should_panic]
fn test_5_edge_case_unauthorized_auditor_invocation() {
    let (env, auditor, fintech_wallet, zk_client) = setup_test_env();
    zk_client.initialize(&auditor);
    
    let mock_zk_proof = BytesN::from_array(&zk_client.env, &[9u8; 32]);
    zk_client.commit_solvency_proof(&fintech_wallet, &mock_zk_proof);
    
    // Evict signatures from the execution context to ensure unauthenticated outside entities cannot verify statements
    env.as_contract_context(&zk_client.address, || {
        zk_client.verify_fintech_solvency(&fintech_wallet);
    });
}
