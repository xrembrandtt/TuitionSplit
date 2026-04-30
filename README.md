# TuitionSplit
A Soroban-based tuition contribution contract that lets students collect and track support from multiple family sponsors in one transparent payment pool.

## Problem
A college student in Manila receives tuition support from different family members, but payments arrive through separate channels and are difficult to track, causing delays, confusion, and missed school payment deadlines.

## Solution
TuitionSplit uses a Soroban smart contract on Stellar to create one shared tuition fund where multiple sponsors can contribute on-chain, while the student can instantly track the total amount collected and the remaining balance needed.

## Suggested Timeline
Day 1: Project setup and contract structure  
Day 2: Implement initialize and contribute functions  
Day 3: Add total, remaining, and sponsor tracking  
Day 4: Write tests and fix errors  
Day 5: Deploy to testnet and prepare demo

## Stellar Features Used
- Soroban smart contracts
- Transparent on-chain contribution tracking
- Financial coordination for split tuition support

## Vision and Purpose
TuitionSplit is designed to make tuition support more organized for students who depend on multiple family sponsors. Instead of relying on screenshots, chat messages, and manual tracking, the contract creates one trusted source of truth for a student’s tuition goal.

## Prerequisites
- Rust toolchain
- Soroban CLI
- Stellar testnet account

## Future Scope
TBA

## How to Build
```bash
soroban contract build