use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::caller;
use ic_cdk_macros::*;
use std::collections::HashMap;
use serde::Serialize;
use num_traits::cast::ToPrimitive;

const STAKING_RATE: f64 = 0.10;
const BORROW_RATE: f64 = 0.12;
const LENDING_REWARD: f64 = 0.05;
const YIELD_FARMING_REWARD: f64 = 0.15;
const SECONDS_IN_YEAR: u64 = 31_536_000;

// Use Principal::from_text for compile-time checked canister ID
const CKBTC_CANISTER_ID_STR: &str = "mxzaz-hqaaa-aaaar-qaada-cai";
lazy_static::lazy_static! {
    static ref CKBTC_CANISTER_ID: Principal = Principal::from_text(CKBTC_CANISTER_ID_STR).expect("Invalid ckBTC canister ID");
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
struct UserData {
    user_principal: Principal,
    ckbtc_balance: u64,
    loans: u64,
    staked: u64,
    lent: u64,
    farmed: u64,
    stake_timestamp: Option<u64>,
    lend_timestamp: Option<u64>,
    farm_timestamp: Option<u64>,
}

#[derive(Default)]
struct State {
    users: HashMap<Principal, UserData>,
}

static mut STATE: Option<State> = None;

fn state() -> &'static mut State {
    unsafe {
        if STATE.is_none() {
            STATE = Some(State::default());
        }
        STATE.as_mut().unwrap()
    }
}

#[init]
fn init() {
    ic_cdk::println!("DeFi backend initialized");
}

#[query]
fn whoami() -> Principal {
    caller()
}

#[update]
fn register_user() -> String {
    let user = caller();
    let s = state();
    if !s.users.contains_key(&user) {
        s.users.insert(user, UserData {
            user_principal: user,
            ckbtc_balance: 0,
            loans: 0,
            staked: 0,
            lent: 0,
            farmed: 0,
            stake_timestamp: None,
            lend_timestamp: None,
            farm_timestamp: None,
        });
        "User registered successfully".to_string()
    } else {
        "User already exists".to_string()
    }
}

#[query]
fn get_user_data(p: Principal) -> Option<UserData> {
    state().users.get(&p).cloned()
}

#[update]
fn deposit_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            data.ckbtc_balance += amount;
            format!("Deposited {} ckBTC", amount)
        }
        None => "User not found. Register first.".to_string(),
    }
}

#[update]
fn borrow_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.ckbtc_balance >= amount / 2 {
                data.loans += amount;
                data.ckbtc_balance += amount;
                format!("Borrowed {} ckBTC", amount)
            } else {
                "Insufficient collateral".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn repay_loan_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.ckbtc_balance >= amount && data.loans >= amount {
                data.loans -= amount;
                data.ckbtc_balance -= amount;
                format!("Repaid {} ckBTC loan", amount)
            } else {
                "Insufficient balance or loan amount too high".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn stake_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.ckbtc_balance >= amount {
                data.ckbtc_balance -= amount;
                data.staked += amount;
                data.stake_timestamp = Some(ic_cdk::api::time());
                format!("Staked {} ckBTC", amount)
            } else {
                "Not enough balance".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn unstake_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.staked >= amount {
                let reward = calculate_interest(data.staked, data.stake_timestamp, STAKING_RATE);
                data.ckbtc_balance += reward + amount;
                data.staked -= amount;
                data.stake_timestamp = Some(ic_cdk::api::time());
                format!("Unstaked {} ckBTC, claimed {} reward", amount, reward)
            } else {
                "Not enough staked amount".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn claim_staking_rewards() -> String {
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let interest = calculate_interest(data.staked, data.stake_timestamp, STAKING_RATE);
            data.ckbtc_balance += interest;
            data.stake_timestamp = Some(ic_cdk::api::time());
            format!("Claimed {} ckBTC as staking rewards", interest)
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn lend_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.ckbtc_balance >= amount {
                data.ckbtc_balance -= amount;
                data.lent += amount;
                data.lend_timestamp = Some(ic_cdk::api::time());
                format!("Lent {} ckBTC", amount)
            } else {
                "Insufficient balance to lend".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn unlend_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.lent >= amount {
                let reward = calculate_interest(data.lent, data.lend_timestamp, LENDING_REWARD);
                data.ckbtc_balance += reward + amount;
                data.lent -= amount;
                data.lend_timestamp = Some(ic_cdk::api::time());
                format!("Unlent {} ckBTC, claimed {} reward", amount, reward)
            } else {
                "Not enough lent amount".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn claim_lending_rewards() -> String {
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let reward = calculate_interest(data.lent, data.lend_timestamp, LENDING_REWARD);
            data.ckbtc_balance += reward;
            data.lend_timestamp = Some(ic_cdk::api::time());
            format!("Claimed {} ckBTC as lending rewards", reward)
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn yield_farm_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.ckbtc_balance >= amount {
                data.ckbtc_balance -= amount;
                data.farmed += amount;
                data.farm_timestamp = Some(ic_cdk::api::time());
                format!("Farming started with {} ckBTC", amount)
            } else {
                "Insufficient balance to farm".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn unfarm_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.farmed >= amount {
                let reward = calculate_interest(data.farmed, data.farm_timestamp, YIELD_FARMING_REWARD);
                data.ckbtc_balance += reward + amount;
                data.farmed -= amount;
                data.farm_timestamp = Some(ic_cdk::api::time());
                format!("Unfarmed {} ckBTC, claimed {} reward", amount, reward)
            } else {
                "Not enough farmed amount".to_string()
            }
        }
        None => "User not found".to_string(),
    }
}

#[update]
fn claim_yield_farming_rewards() -> String {
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let reward = calculate_interest(data.farmed, data.farm_timestamp, YIELD_FARMING_REWARD);
            data.ckbtc_balance += reward;
            data.farm_timestamp = Some(ic_cdk::api::time());
            format!("Claimed {} ckBTC as yield farming rewards", reward)
        }
        None => "User not found".to_string(),
    }
}

fn calculate_interest(amount: u64, timestamp: Option<u64>, rate: f64) -> u64 {
    if let Some(start_time) = timestamp {
        let now = ic_cdk::api::time();
        // Convert nanoseconds to seconds for correct interest calculation
        let seconds = (now - start_time) / 1_000_000_000;
        let interest = (amount as f64) * (rate * (seconds as f64 / SECONDS_IN_YEAR as f64));
        interest as u64
    } else {
        0
    }
}

#[derive(CandidType, Deserialize, Clone)]
struct TransferArg {
    to: Principal,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
struct TransferResult {
    Ok: Option<Nat>,
    Err: Option<String>,
}

#[update]
async fn transfer_ckbtc(to: Principal, amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    // Plug wallet users must approve this canister to transfer on their behalf, or transfer from Plug directly.
    let transfer_arg = TransferArg {
        to,
        amount: Nat::from(amount),
        fee: None,
        memo: None,
        from_subaccount: None, // For advanced flows, set subaccount if needed
        created_at_time: None,
    };

    // Use the static principal
    let result: Result<(TransferResult,), _> = ic_cdk::call(
        *CKBTC_CANISTER_ID,
        "icrc1_transfer",
        (transfer_arg,)
    ).await;

    match result {
        Ok((TransferResult { Ok: Some(tx_id), .. },)) => format!("Transferred! Tx ID: {}", tx_id),
        Ok((TransferResult { Err: Some(err), .. },)) => format!("Transfer failed: {}", err),
        Err(e) => format!("Call failed: {:?}", e),
        _ => "Unknown error".to_string(),
    }
}

#[update]
async fn withdraw_ckbtc(amount: u64) -> String {
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.ckbtc_balance >= amount {
                data.ckbtc_balance -= amount;
                // This will call the real ckBTC canister to transfer tokens to the user
                transfer_ckbtc(user, amount).await
            } else {
                "Not enough balance to withdraw".to_string()
            }
        }
        None => "User not registered".to_string(),
    }
}

// (Optional) Function to fetch real balance from ckBTC canister
#[update]
async fn get_real_ckbtc_balance() -> Result<u64, String> {
    let user = caller();
    // Use the static principal
    let result: Result<(Nat,), _> = ic_cdk::call(
        *CKBTC_CANISTER_ID,
        "icrc1_balance_of",
        (Record { owner: user },)
    ).await;

    match result {
        Ok((balance_nat,)) => {
            let balance_u64 = balance_nat.0.to_u64().unwrap_or(0);
            Ok(balance_u64)
        }
        Err(e) => Err(format!("Failed to fetch ckBTC balance: {:?}", e)),
    }
}

#[derive(CandidType, Deserialize)]
struct Record {
    owner: Principal,
}
