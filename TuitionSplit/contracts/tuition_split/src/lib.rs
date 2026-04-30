#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contract]
pub struct TuitionSplit;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Student,
    GoalName,
    TargetAmount,
    TotalCollected,
    Contribution(Address),
}

#[contractimpl]
impl TuitionSplit {
    // Creates the tuition fund and stores the owner, fund name, and target amount.
    pub fn initialize_fund(env: Env, student: Address, goal_name: String, target_amount: i128) {
        if env.storage().instance().has(&DataKey::Student) {
            panic!("fund already initialized");
        }

        if target_amount <= 0 {
            panic!("target amount must be greater than zero");
        }

        student.require_auth();

        env.storage().instance().set(&DataKey::Student, &student);
        env.storage().instance().set(&DataKey::GoalName, &goal_name);
        env.storage().instance().set(&DataKey::TargetAmount, &target_amount);
        env.storage().instance().set(&DataKey::TotalCollected, &0i128);
    }

    // Lets any sponsor contribute an amount to the shared tuition fund.
    pub fn contribute(env: Env, sponsor: Address, amount: i128) {
        if !env.storage().instance().has(&DataKey::Student) {
            panic!("fund not initialized");
        }

        if amount <= 0 {
            panic!("amount must be greater than zero");
        }

        sponsor.require_auth();

        let current_total: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalCollected)
            .unwrap_or(0i128);

        let new_total = current_total + amount;
        env.storage().instance().set(&DataKey::TotalCollected, &new_total);

        let sponsor_key = DataKey::Contribution(sponsor.clone());
        let previous_contribution: i128 = env
            .storage()
            .persistent()
            .get(&sponsor_key)
            .unwrap_or(0i128);

        env.storage()
            .persistent()
            .set(&sponsor_key, &(previous_contribution + amount));
    }

    // Returns the total amount collected so far.
    pub fn get_total(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalCollected)
            .unwrap_or(0i128)
    }

    // Returns the target tuition amount.
    pub fn get_target(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TargetAmount)
            .unwrap_or(0i128)
    }

    // Returns the remaining balance needed to reach the tuition goal.
    pub fn get_remaining(env: Env) -> i128 {
        let target: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TargetAmount)
            .unwrap_or(0i128);

        let total: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalCollected)
            .unwrap_or(0i128);

        if total >= target {
            0
        } else {
            target - total
        }
    }

    // Returns how much a specific sponsor has contributed.
    pub fn get_contribution(env: Env, sponsor: Address) -> i128 {
        let sponsor_key = DataKey::Contribution(sponsor);
        env.storage().persistent().get(&sponsor_key).unwrap_or(0i128)
    }

    // Returns true if the tuition goal has already been reached.
    pub fn is_fully_funded(env: Env) -> bool {
        let target = Self::get_target(env.clone());
        let total = Self::get_total(env);
        total >= target && target > 0
    }
}