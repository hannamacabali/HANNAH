#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_happy_path_end_to_end() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RiceRouteContract);
    let client = RiceRouteContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let coop = Address::generate(&env);
    let hauler = Address::generate(&env);
    let allocation_amount = 500_i128;

    // Step 1: Init Route
    client.init_route(&farmer, &coop, &hauler, &allocation_amount);

    // Step 2: Fund Transport
    client.fund_transport();

    // Step 3: Complete Delivery
    client.complete_delivery();

    let state = client.get_state();
    assert!(state.is_completed);
    assert!(state.is_funded);
}

#[test]
#[should_panic(expected = "Agreement already funded")]
fn test_edge_case_duplicate_funding_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RiceRouteContract);
    let client = RiceRouteContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let coop = Address::generate(&env);
    let hauler = Address::generate(&env);

    client.init_route(&farmer, &coop, &hauler, &500_i128);
    client.fund_transport();
    
    // Direct second call triggers panic
    client.fund_transport();
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RiceRouteContract);
    let client = RiceRouteContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let coop = Address::generate(&env);
    let hauler = Address::generate(&env);

    client.init_route(&farmer, &coop, &hauler, &1000_i128);
    
    let state = client.get_state();
    assert_eq!(state.farmer, farmer);
    assert_eq!(state.amount, 1000_i128);
    assert!(!state.is_funded);
}

#[test]
#[should_panic(expected = "Cannot complete an unfunded route")]
fn test_edge_case_completion_without_funding_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, RiceRouteContract);
    let client = RiceRouteContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let coop = Address::generate(&env);
    let hauler = Address::generate(&env);

    client.init_route(&farmer, &coop, &hauler, &500_i128);
    
    // Trying to complete before funding should fail
    client.complete_delivery();
}

#[test]
#[should_panic]
fn test_unauthorized_farmer_initialization_fails() {
    let env = Env::default();
    // Intentionally omitting mock_all_auths to verify structural security flags
    let contract_id = env.register_contract(None, RiceRouteContract);
    let client = RiceRouteContractClient::new(&env, &contract_id);

    let farmer = Address::generate(&env);
    let coop = Address::generate(&env);
    let hauler = Address::generate(&env);

    client.init_route(&farmer, &coop, &hauler, &500_i128);
}