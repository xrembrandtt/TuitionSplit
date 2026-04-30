#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct TuitionSplit;

#[contractimpl]
impl TuitionSplit {
    pub fn hello(_env: Env) -> u32 {
        1
    }
}