#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_happy_path_contribution() {
    let env = Env::default();
    let contract_id = env.register(TuitionSplit, ());
    let client = TuitionSplitClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let sponsor = Address::generate(&env);

    client.initialize_fund(&student, &String::from_str(&env, "Tuition 2026"), &10000);
    client.contribute(&sponsor, &3000);

    assert_eq!(client.get_total(), 3000);
    assert_eq!(client.get_remaining(), 7000);
}

#[test]
#[should_panic(expected = "amount must be greater than zero")]
fn test_edge_case_zero_contribution_rejected() {
    let env = Env::default();
    let contract_id = env.register(TuitionSplit, ());
    let client = TuitionSplitClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let sponsor = Address::generate(&env);

    client.initialize_fund(&student, &String::from_str(&env, "Tuition 2026"), &10000);
    client.contribute(&sponsor, &0);
}

#[test]
fn test_state_verification_after_contribution() {
    let env = Env::default();
    let contract_id = env.register(TuitionSplit, ());
    let client = TuitionSplitClient::new(&env, &contract_id);

    let student = Address::generate(&env);
    let sponsor = Address::generate(&env);

    client.initialize_fund(&student, &String::from_str(&env, "Tuition 2026"), &15000);
    client.contribute(&sponsor, &5000);

    assert_eq!(client.get_target(), 15000);
    assert_eq!(client.get_total(), 5000);
    assert_eq!(client.get_contribution(&sponsor), 5000);
    assert_eq!(client.is_fully_funded(), false);
}