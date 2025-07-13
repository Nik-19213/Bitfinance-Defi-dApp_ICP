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

const COLLATERAL_RATIO: f64 = 2.0;

const CKBTC_TRANSFER_FEE: u64 = 10; 

// Canister IDs for different networks
const CKBTC_MAINNET_CANISTER_ID: &str = "mxzaz-hqaaa-aaaar-qaada-cai";
const CKBTC_TESTNET_CANISTER_ID: &str = "mc6ru-gyaaa-aaaar-qaaaq-cai";   //use uxrrr-q7777-77774-qaaaq-cai for local deployemnt

const IS_TESTNET: bool = true;

lazy_static::lazy_static! {
    static ref CKBTC_CANISTER_ID: Principal = Principal::from_text(
        if IS_TESTNET { CKBTC_TESTNET_CANISTER_ID } else { CKBTC_MAINNET_CANISTER_ID }
    ).expect("Invalid ckBTC canister ID");
}

// ICRC-1 and ICRC-2 Standard Types
#[derive(CandidType, Deserialize, Clone)]
struct Account {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone)]
struct TransferArg {
    from_subaccount: Option<Vec<u8>>,
    to: Account,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone)]
struct TransferFromArg {
    spender_subaccount: Option<Vec<u8>>,
    from: Account,
    to: Account,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>, 
}

#[derive(CandidType, Deserialize, Clone)]
struct ApproveArg {
    from_subaccount: Option<Vec<u8>>,
    spender: Account,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    expires_at: Option<u64>,
    expected_allowance: Option<Nat>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
enum TransferError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize, Clone, Debug)]
enum TransferFromError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    InsufficientAllowance { allowance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize, Clone, Debug)]
enum ApproveError {
    BadFee { expected_fee: Nat },
    InsufficientFunds { balance: Nat },
    AllowanceChanged { current_allowance: Nat },
    Expired { ledger_time: u64 },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: Nat },
    TemporarilyUnavailable,
    GenericError { error_code: Nat, message: String },
}

// User data structure
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
    loan_timestamp: Option<u64>,
}

// Global state
#[derive(Default)]
struct State {
    users: HashMap<Principal, UserData>,
    is_paused: bool,
    admin: Option<Principal>,
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

// Initialization
#[init]
fn init() {
    let s = state();
    s.admin = Some(caller());
    ic_cdk::println!("DeFi backend initialized on {}", if IS_TESTNET { "testnet" } else { "mainnet" });
}

// Admin functions
#[update]
fn pause_contract() -> String {
    let s = state();
    if Some(caller()) != s.admin {
        return "Unauthorized: Only admin can pause".to_string();
    }
    s.is_paused = true;
    "Contract paused".to_string()
}

#[update]
fn unpause_contract() -> String {
    let s = state();
    if Some(caller()) != s.admin {
        return "Unauthorized: Only admin can unpause".to_string();
    }
    s.is_paused = false;
    "Contract unpaused".to_string()
}

// Modifier to check if contract is paused
fn ensure_not_paused() -> Result<(), String> {
    if state().is_paused {
        Err("Contract is paused".to_string())
    } else {
        Ok(())
    }
}

// Basic functions
#[query]
fn whoami() -> Principal {
    caller()
}

#[query]
fn get_contract_info() -> String {
    format!(
        "DeFi Contract - Network: {}, Paused: {}, ckBTC Canister: {}",
        if IS_TESTNET { "Testnet" } else { "Mainnet" },
        state().is_paused,
        CKBTC_CANISTER_ID.to_text()
    )
}

#[update]
fn register_user() -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
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
            loan_timestamp: None,
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

#[query]
fn get_my_data() -> Option<UserData> {
    state().users.get(&caller()).cloned()
}

// Helper function to calculate interest/rewards
fn calculate_interest(amount: u64, timestamp: Option<u64>, rate: f64) -> u64 {
    if let Some(start_time) = timestamp {
        let now = ic_cdk::api::time();
        let seconds = (now - start_time) / 1_000_000_000;
        let annual_interest = (amount as f64) * rate;
        let interest = annual_interest * (seconds as f64 / SECONDS_IN_YEAR as f64);
        interest as u64
    } else {
        0
    }
}

#[derive(CandidType, Deserialize)]
struct AllowanceArgs {
    account: Account,
    spender: Account,
}

#[derive(CandidType, Deserialize)]
struct Allowance {
    allowance: Nat,
    expires_at: Option<u64>,
}

#[update]
async fn check_allowance(owner: Principal) -> Result<Allowance, String> {
    let args = AllowanceArgs {
        account: Account {
            owner,
            subaccount: None,
        },
        spender: Account {
            owner: ic_cdk::id(),
            subaccount: None,
        },
    };

    let result: Result<(Allowance,), _> = ic_cdk::call(
        *CKBTC_CANISTER_ID,
        "icrc2_allowance",
        (args,),
    ).await;

    match result {
        Ok((allowance,)) => Ok(allowance),
        Err(e) => Err(format!("Failed to check allowance: {:?}", e)),
    }
}

// Get real ckBTC balance from the ledger
#[update]
async fn get_real_ckbtc_balance(owner: Option<Principal>) -> Result<u64, String> {
    let account_owner = owner.unwrap_or(caller());
    let account = Account {
        owner: account_owner,
        subaccount: None,
    };

    let result: Result<(Nat,), _> = ic_cdk::call(
        *CKBTC_CANISTER_ID,
        "icrc1_balance_of",
        (account,)
    ).await;

    match result {
        Ok((balance,)) => Ok(balance.0.to_u64().unwrap_or(0)),
        Err(e) => Err(format!("Failed to fetch ckBTC balance: {:?}", e)),
    }
}

// Core transfer functions
async fn transfer_ckbtc_from_user_to_canister(from: Principal, amount: u64) -> Result<Nat, String> {
    let transfer_from_arg = TransferFromArg {
        spender_subaccount: None,
        from: Account { owner: from, subaccount: None },
        to: Account { owner: ic_cdk::id(), subaccount: None },
        amount: Nat::from(amount),
        fee: Some(Nat::from(CKBTC_TRANSFER_FEE)),
        memo: None,
        created_at_time: Some(ic_cdk::api::time()),
    };

    let result: Result<(Result<Nat, TransferFromError>,), _> = ic_cdk::call(
        *CKBTC_CANISTER_ID,
        "icrc2_transfer_from",
        (transfer_from_arg,)
    ).await;

    match result {
        Ok((Ok(tx_id),)) => Ok(tx_id),
        Ok((Err(TransferFromError::InsufficientAllowance { allowance }),)) => {
            Err(format!("Insufficient allowance. Current allowance: {} ckBTC. Please approve more tokens in your Plug wallet.", allowance))
        }
        Ok((Err(TransferFromError::InsufficientFunds { balance }),)) => {
            Err(format!("Insufficient funds. Balance: {} ckBTC", balance))
        }
        Ok((Err(e),)) => Err(format!("Transfer failed: {:?}", e)),
        Err(e) => Err(format!("Call failed: {:?}", e)),
    }
}

async fn transfer_ckbtc_from_canister_to_user(to: Principal, amount: u64) -> Result<Nat, String> {
    let transfer_arg = TransferArg {
        from_subaccount: None,
        to: Account { owner: to, subaccount: None },
        amount: Nat::from(amount),
        fee: Some(Nat::from(CKBTC_TRANSFER_FEE)),
        memo: None,
        created_at_time: Some(ic_cdk::api::time()),
    };

    let result: Result<(Result<Nat, TransferError>,), _> = ic_cdk::call(
        *CKBTC_CANISTER_ID,
        "icrc1_transfer",
        (transfer_arg,)
    ).await;

    match result {
        Ok((Ok(tx_id),)) => Ok(tx_id),
        Ok((Err(TransferError::InsufficientFunds { balance }),)) => {
            Err(format!("Canister has insufficient funds. Balance: {} ckBTC", balance))
        }
        Ok((Err(e),)) => Err(format!("Transfer failed: {:?}", e)),
        Err(e) => Err(format!("Call failed: {:?}", e)),
    }
}

// Helper to convert ckBTC (float) to satoshis (u64)
fn ckbtc_to_sats(amount: f64) -> u64 {
    (amount * 100_000_000.0).round() as u64
}

// Deposit ckBTC (user must approve first)
#[update]
async fn deposit_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let total_required = sats + CKBTC_TRANSFER_FEE;
            match check_allowance(user).await {
                Ok(allowance) => {
                    if allowance.allowance.0.to_u64().unwrap_or(0) < total_required {
                        return format!("Insufficient allowance. Please approve {} ckBTC in your Plug wallet first.", (total_required as f64) / 100_000_000.0);
                    }
                }
                Err(e) => return format!("Failed to check allowance: {}", e),
            }
            match transfer_ckbtc_from_user_to_canister(user, total_required).await {
                Ok(tx_id) => {
                    data.ckbtc_balance += sats;
                    format!("Deposited {:.8} ckBTC (fee: {:.8} ckBTC). Transaction ID: {}", amount, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, tx_id)
                }
                Err(e) => format!("Deposit failed: {}", e),
            }
        }
        None => "User not registered. Please register first.".to_string(),
    }
}

// Withdraw ckBTC
#[update]
async fn withdraw_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let total_required = sats + CKBTC_TRANSFER_FEE;
            if data.ckbtc_balance < total_required {
                return format!("Insufficient balance. You have {:.8} ckBTC, need {:.8} (including {:.8} fee)", 
                    (data.ckbtc_balance as f64)/100_000_000.0, (total_required as f64)/100_000_000.0, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0);
            }
            match transfer_ckbtc_from_canister_to_user(user, sats).await {
                Ok(tx_id) => {
                    data.ckbtc_balance -= total_required;
                    format!("Withdrew {:.8} ckBTC (fee: {:.8} ckBTC). Transaction ID: {}", amount, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, tx_id)
                }
                Err(e) => format!("Withdrawal failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Borrow ckBTC (requires collateral)
#[update]
async fn borrow_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let required_collateral = (sats as f64 * COLLATERAL_RATIO) as u64;
            let available_collateral = data.ckbtc_balance + data.staked + data.lent + data.farmed;
            if available_collateral < required_collateral {
                return format!("Insufficient collateral. Required: {:.8} ckBTC, Available: {:.8} ckBTC", 
                    (required_collateral as f64)/100_000_000.0, (available_collateral as f64)/100_000_000.0);
            }
            match transfer_ckbtc_from_canister_to_user(user, sats).await {
                Ok(tx_id) => {
                    data.loans += sats;
                    data.loan_timestamp = Some(ic_cdk::api::time());
                    data.ckbtc_balance += sats;
                    format!("Borrowed {:.8} ckBTC. Transaction ID: {}. Remember to repay with {}% annual interest.", 
                        amount, tx_id, (BORROW_RATE * 100.0) as u32)
                }
                Err(e) => format!("Borrow failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Repay loan
#[update]
async fn repay_loan_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.loans == 0 {
                return "No active loans to repay".to_string();
            }
            let interest = calculate_interest(data.loans, data.loan_timestamp, BORROW_RATE);
            let total_debt = data.loans + interest;
            if sats > total_debt {
                return format!("Amount exceeds total debt. Total debt (principal + interest): {:.8} ckBTC", (total_debt as f64)/100_000_000.0);
            }
            let total_required = sats + CKBTC_TRANSFER_FEE;
            match check_allowance(user).await {
                Ok(allowance) => {
                    if allowance.allowance.0.to_u64().unwrap_or(0) < total_required {
                        return format!("Insufficient allowance. Please approve {:.8} ckBTC in your Plug wallet first.", (total_required as f64)/100_000_000.0);
                    }
                }
                Err(e) => return format!("Failed to check allowance: {}", e),
            }
            match transfer_ckbtc_from_user_to_canister(user, total_required).await {
                Ok(tx_id) => {
                    if data.ckbtc_balance < total_required {
                        return format!("Insufficient ckBTC balance to repay loan. You need {:.8} (amount + fee), have {:.8}.", (total_required as f64)/100_000_000.0, (data.ckbtc_balance as f64)/100_000_000.0);
                    }
                    data.ckbtc_balance -= total_required;
                    if sats >= data.loans {
                        data.loans = 0;
                        data.loan_timestamp = None;
                        format!("Loan fully repaid. Transaction ID: {}", tx_id)
                    } else {
                        data.loans -= sats;
                        data.loan_timestamp = Some(ic_cdk::api::time());
                        format!("Partial repayment of {:.8} ckBTC. Remaining debt: {:.8} ckBTC. Transaction ID: {}", 
                            amount, (data.loans as f64)/100_000_000.0, tx_id)
                    }
                }
                Err(e) => format!("Repayment failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Stake ckBTC
#[update]
async fn stake_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let total_required = sats + CKBTC_TRANSFER_FEE;
            if data.ckbtc_balance < total_required {
                return format!("Insufficient ckBTC balance. You need {:.8} (amount + fee), have {:.8}.", (total_required as f64)/100_000_000.0, (data.ckbtc_balance as f64)/100_000_000.0);
            }
            data.ckbtc_balance -= total_required;
            data.staked += sats;
            data.stake_timestamp = Some(ic_cdk::api::time());
            format!("Staked {:.8} ckBTC (fee: {:.8}). Earning {}% annual rewards.", 
                amount, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, (STAKING_RATE * 100.0) as u32)
        }
        None => "User not registered".to_string(),
    }
}

// Unstake ckBTC
#[update]
async fn unstake_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let rewards = calculate_interest(data.staked, data.stake_timestamp, STAKING_RATE);
            let total_required = sats + CKBTC_TRANSFER_FEE;
            if data.staked < total_required {
                return format!("Insufficient staked amount. You need {:.8} (amount + fee), have {:.8}.", (total_required as f64)/100_000_000.0, (data.staked as f64)/100_000_000.0);
            }
            let total_to_send = sats + rewards;
            match transfer_ckbtc_from_canister_to_user(user, total_to_send).await {
                Ok(tx_id) => {
                    data.staked -= total_required;
                    data.ckbtc_balance += sats + rewards;
                    if data.staked == 0 {
                        data.stake_timestamp = None;
                    } else {
                        data.stake_timestamp = Some(ic_cdk::api::time());
                    }
                    format!("Unstaked {:.8} ckBTC + {:.8} rewards (fee: {:.8}). Transaction ID: {}", amount, (rewards as f64)/100_000_000.0, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, tx_id)
                }
                Err(e) => format!("Unstaking failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Lend ckBTC
#[update]
async fn lend_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let total_required = sats + CKBTC_TRANSFER_FEE;
            if data.ckbtc_balance < total_required {
                return format!("Insufficient ckBTC balance. You need {:.8} (amount + fee), have {:.8}.", (total_required as f64)/100_000_000.0, (data.ckbtc_balance as f64)/100_000_000.0);
            }
            data.ckbtc_balance -= total_required;
            data.lent += sats;
            data.lend_timestamp = Some(ic_cdk::api::time());
            format!("Lent {:.8} ckBTC (fee: {:.8}). Earning {}% annual rewards.", 
                amount, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, (LENDING_REWARD * 100.0) as u32)
        }
        None => "User not registered".to_string(),
    }
}

// Unlend ckBTC
#[update]
async fn unlend_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let rewards = calculate_interest(data.lent, data.lend_timestamp, LENDING_REWARD);
            let total_required = sats + CKBTC_TRANSFER_FEE;
            if data.lent < total_required {
                return format!("Insufficient lent amount. You need {:.8} (amount + fee), have {:.8}.", (total_required as f64)/100_000_000.0, (data.lent as f64)/100_000_000.0);
            }
            let total_to_send = sats + rewards;
            match transfer_ckbtc_from_canister_to_user(user, total_to_send).await {
                Ok(tx_id) => {
                    data.lent -= total_required;
                    data.ckbtc_balance += sats + rewards;
                    if data.lent == 0 {
                        data.lend_timestamp = None;
                    } else {
                        data.lend_timestamp = Some(ic_cdk::api::time());
                    }
                    format!("Unlent {:.8} ckBTC + {:.8} rewards (fee: {:.8}). Transaction ID: {}", amount, (rewards as f64)/100_000_000.0, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, tx_id)
                }
                Err(e) => format!("Unlending failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Yield farm ckBTC
#[update]
async fn yield_farm_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let total_required = sats + CKBTC_TRANSFER_FEE;
            if data.ckbtc_balance < total_required {
                return format!("Insufficient ckBTC balance. You need {:.8} (amount + fee), have {:.8}.", (total_required as f64)/100_000_000.0, (data.ckbtc_balance as f64)/100_000_000.0);
            }
            data.ckbtc_balance -= total_required;
            data.farmed += sats;
            data.farm_timestamp = Some(ic_cdk::api::time());
            format!("Started yield farming with {:.8} ckBTC (fee: {:.8}). Earning {}% annual rewards.", 
                amount, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, (YIELD_FARMING_REWARD * 100.0) as u32)
        }
        None => "User not registered".to_string(),
    }
}

// Stop yield farming
#[update]
async fn unfarm_ckbtc(amount: f64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    if amount <= 0.0 {
        return "Amount must be greater than 0".to_string();
    }
    let sats = ckbtc_to_sats(amount);
    let user = caller();
    let s = state();
    match s.users.get_mut(&user) {
        Some(data) => {
            let rewards = calculate_interest(data.farmed, data.farm_timestamp, YIELD_FARMING_REWARD);
            let total_required = sats + CKBTC_TRANSFER_FEE;
            if data.farmed < total_required {
                return format!("Insufficient farmed amount. You need {:.8} (amount + fee), have {:.8}.", (total_required as f64)/100_000_000.0, (data.farmed as f64)/100_000_000.0);
            }
            let total_to_send = sats + rewards;
            match transfer_ckbtc_from_canister_to_user(user, total_to_send).await {
                Ok(tx_id) => {
                    data.farmed -= total_required;
                    data.ckbtc_balance += sats + rewards;
                    if data.farmed == 0 {
                        data.farm_timestamp = None;
                    } else {
                        data.farm_timestamp = Some(ic_cdk::api::time());
                    }
                    format!("Stopped farming {:.8} ckBTC + {:.8} rewards (fee: {:.8}). Transaction ID: {}", amount, (rewards as f64)/100_000_000.0, (CKBTC_TRANSFER_FEE as f64)/100_000_000.0, tx_id)
                }
                Err(e) => format!("Unfarming failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Claim individual rewards functions
#[update]
async fn claim_staking_rewards() -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            let rewards = calculate_interest(data.staked, data.stake_timestamp, STAKING_RATE);
            if rewards == 0 {
                return "No staking rewards to claim".to_string();
            }
            // User must have enough staked to cover the fee
            if data.staked < CKBTC_TRANSFER_FEE {
                return format!("Insufficient staked amount to cover the fee. Need at least {} staked.", CKBTC_TRANSFER_FEE);
            }
            match transfer_ckbtc_from_canister_to_user(user, rewards).await {
                Ok(tx_id) => {
                    data.staked -= CKBTC_TRANSFER_FEE;
                    data.ckbtc_balance += rewards;
                    data.stake_timestamp = Some(ic_cdk::api::time());
                    format!("Claimed {} ckBTC as staking rewards (fee: {}). Transaction ID: {}", rewards, CKBTC_TRANSFER_FEE, tx_id)
                }
                Err(e) => format!("Claim failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

#[update]
async fn claim_lending_rewards() -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            let rewards = calculate_interest(data.lent, data.lend_timestamp, LENDING_REWARD);
            if rewards == 0 {
                return "No lending rewards to claim".to_string();
            }
            if data.lent < CKBTC_TRANSFER_FEE {
                return format!("Insufficient lent amount to cover the fee. Need at least {} lent.", CKBTC_TRANSFER_FEE);
            }
            match transfer_ckbtc_from_canister_to_user(user, rewards).await {
                Ok(tx_id) => {
                    data.lent -= CKBTC_TRANSFER_FEE;
                    data.ckbtc_balance += rewards;
                    data.lend_timestamp = Some(ic_cdk::api::time());
                    format!("Claimed {} ckBTC as lending rewards (fee: {}). Transaction ID: {}", rewards, CKBTC_TRANSFER_FEE, tx_id)
                }
                Err(e) => format!("Claim failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

#[update]
async fn claim_yield_farming_rewards() -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            let rewards = calculate_interest(data.farmed, data.farm_timestamp, YIELD_FARMING_REWARD);
            if rewards == 0 {
                return "No yield farming rewards to claim".to_string();
            }
            if data.farmed < CKBTC_TRANSFER_FEE {
                return format!("Insufficient farmed amount to cover the fee. Need at least {} farmed.", CKBTC_TRANSFER_FEE);
            }
            match transfer_ckbtc_from_canister_to_user(user, rewards).await {
                Ok(tx_id) => {
                    data.farmed -= CKBTC_TRANSFER_FEE;
                    data.ckbtc_balance += rewards;
                    data.farm_timestamp = Some(ic_cdk::api::time());
                    format!("Claimed {} ckBTC as yield farming rewards (fee: {}). Transaction ID: {}", rewards, CKBTC_TRANSFER_FEE, tx_id)
                }
                Err(e) => format!("Claim failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Emergency functions
#[update]
async fn emergency_withdraw_all() -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.loans > 0 {
                return "Cannot withdraw all while having active loans. Please repay loans first.".to_string();
            }

            let total_staked = data.staked;
            let total_lent = data.lent;
            let total_farmed = data.farmed;
            let balance = data.ckbtc_balance;

            let staking_rewards = calculate_interest(data.staked, data.stake_timestamp, STAKING_RATE);
            let lending_rewards = calculate_interest(data.lent, data.lend_timestamp, LENDING_REWARD);
            let farming_rewards = calculate_interest(data.farmed, data.farm_timestamp, YIELD_FARMING_REWARD);

            let total_amount = total_staked + total_lent + total_farmed + balance + 
                              staking_rewards + lending_rewards + farming_rewards;

            if total_amount <= CKBTC_TRANSFER_FEE {
                return "No funds to withdraw or not enough to cover the fee".to_string();
            }

            let withdrawable = total_amount - CKBTC_TRANSFER_FEE;

            match transfer_ckbtc_from_canister_to_user(user, withdrawable).await {
                Ok(tx_id) => {
                    // Reset all user data
                    data.ckbtc_balance = 0;
                    data.staked = 0;
                    data.lent = 0;
                    data.farmed = 0;
                    data.stake_timestamp = None;
                    data.lend_timestamp = None;
                    data.farm_timestamp = None;

                    format!("Emergency withdrawal successful. Total withdrawn: {} ckBTC (includes all rewards, minus fee). Transaction ID: {}", 
                        withdrawable, tx_id)
                }
                Err(e) => format!("Emergency withdrawal failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Statistics and info functions
#[query]
fn get_platform_stats() -> String {
    let s = state();
    let mut total_deposits = 0u64;
    let mut total_loans = 0u64;
    let mut total_staked = 0u64;
    let mut total_lent = 0u64;
    let mut total_farmed = 0u64;
    let mut user_count = 0u32;

    for (_, data) in s.users.iter() {
        total_deposits += data.ckbtc_balance;
        total_loans += data.loans;
        total_staked += data.staked;
        total_lent += data.lent;
        total_farmed += data.farmed;
        user_count += 1;
    }

    format!(
        "Platform Statistics:\n\
        Network: {}\n\
        Total Users: {}\n\
        Total Deposits: {} ckBTC\n\
        Total Loans: {} ckBTC\n\
        Total Staked: {} ckBTC\n\
        Total Lent: {} ckBTC\n\
        Total Yield Farming: {} ckBTC\n\
        Contract Status: {}",
        if IS_TESTNET { "Testnet" } else { "Mainnet" },
        user_count,
        total_deposits,
        total_loans,
        total_staked,
        total_lent,
        total_farmed,
        if s.is_paused { "Paused" } else { "Active" }
    )
}

// Helper function for Plug wallet integration guide
#[query]
fn get_integration_guide() -> String {
    format!(
        "🔗 Plug Wallet Integration Guide:\n\n\
        1. First, register as a user: `register_user()`\n\
        2. In Plug wallet, approve this canister to spend your ckBTC:\n\
           - Canister ID: {}\n\
           - Use the approve function in your wallet\n\
        3. Then you can use any DeFi function:\n\
           - `deposit_ckbtc(amount)` - Deposit ckBTC\n\
           - `stake_ckbtc(amount)` - Earn {}% annual rewards\n\
           - `lend_ckbtc(amount)` - Earn {}% annual rewards\n\
           - `yield_farm_ckbtc(amount)` - Earn {}% annual rewards\n\
           - `borrow_ckbtc(amount)` - Borrow at {}% annual rate\n\
        4. View your data: `get_my_data()`\n\
        5. Check pending rewards: `get_pending_*_rewards()`\n\n\
        💡 Network: {}\n\
        💡 All amounts are in ckBTC (1 BTC = 100,000,000 ckBTC)\n\
        💡 Transfer fee: {} ckBTC per transaction",
        ic_cdk::id().to_text(),
        (STAKING_RATE * 100.0) as u32,
        (LENDING_REWARD * 100.0) as u32,
        (YIELD_FARMING_REWARD * 100.0) as u32,
        (BORROW_RATE * 100.0) as u32,
        if IS_TESTNET { "Testnet" } else { "Mainnet" },
        CKBTC_TRANSFER_FEE
    )
}