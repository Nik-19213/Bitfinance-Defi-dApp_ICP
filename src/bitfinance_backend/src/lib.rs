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
const CKBTC_TESTNET_CANISTER_ID: &str = "uxrrr-q7777-77774-qaaaq-cai";

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
            Err(format!("Insufficient allowance. Current allowance: {} satoshis. Please approve more tokens in your Plug wallet.", allowance))
        }
        Ok((Err(TransferFromError::InsufficientFunds { balance }),)) => {
            Err(format!("Insufficient funds. Balance: {} satoshis", balance))
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
            Err(format!("Canister has insufficient funds. Balance: {} satoshis", balance))
        }
        Ok((Err(e),)) => Err(format!("Transfer failed: {:?}", e)),
        Err(e) => Err(format!("Call failed: {:?}", e)),
    }
}

// Deposit ckBTC (user must approve first)
#[update]
async fn deposit_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    if amount <= CKBTC_TRANSFER_FEE {
        return format!("Amount must be greater than transfer fee ({} satoshis)", CKBTC_TRANSFER_FEE);
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            // Check allowance first
            match check_allowance(user).await {
                Ok(allowance) => {
                    if allowance.allowance.0.to_u64().unwrap_or(0) < amount {
                        return format!("Insufficient allowance. Please approve {} satoshis in your Plug wallet first.", amount);
                    }
                }
                Err(e) => return format!("Failed to check allowance: {}", e),
            }

            // Transfer ckBTC from user to canister
            match transfer_ckbtc_from_user_to_canister(user, amount).await {
                Ok(tx_id) => {
                    let actual_amount = amount - CKBTC_TRANSFER_FEE;
                    data.ckbtc_balance += actual_amount;
                    format!("Deposited {} satoshis (fee: {} satoshis). Transaction ID: {}", actual_amount, CKBTC_TRANSFER_FEE, tx_id)
                }
                Err(e) => format!("Deposit failed: {}", e),
            }
        }
        None => "User not registered. Please register first.".to_string(),
    }
}

// Withdraw ckBTC
#[update]
async fn withdraw_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            let total_amount = amount + CKBTC_TRANSFER_FEE;
            if data.ckbtc_balance < total_amount {
                return format!("Insufficient balance. You have {} satoshis, need {} (including {} fee)", 
                    data.ckbtc_balance, total_amount, CKBTC_TRANSFER_FEE);
            }

            match transfer_ckbtc_from_canister_to_user(user, amount).await {
                Ok(tx_id) => {
                    data.ckbtc_balance -= total_amount;
                    format!("Withdrew {} satoshis (fee: {} satoshis). Transaction ID: {}", amount, CKBTC_TRANSFER_FEE, tx_id)
                }
                Err(e) => format!("Withdrawal failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Borrow ckBTC (requires collateral)
#[update]
async fn borrow_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            let required_collateral = (amount as f64 * COLLATERAL_RATIO) as u64;
            let available_collateral = data.ckbtc_balance + data.staked + data.lent + data.farmed;
            
            if available_collateral < required_collateral {
                return format!("Insufficient collateral. Required: {} satoshis, Available: {} satoshis", 
                    required_collateral, available_collateral);
            }

            match transfer_ckbtc_from_canister_to_user(user, amount).await {
                Ok(tx_id) => {
                    data.loans += amount;
                    data.loan_timestamp = Some(ic_cdk::api::time());
                    data.ckbtc_balance += amount; // Add borrowed amount to balance
                    format!("Borrowed {} satoshis. Transaction ID: {}. Remember to repay with {}% annual interest.", 
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
async fn repay_loan_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.loans == 0 {
                return "No active loans to repay".to_string();
            }

            let interest = calculate_interest(data.loans, data.loan_timestamp, BORROW_RATE);
            let total_debt = data.loans + interest;
            
            if amount > total_debt {
                return format!("Amount exceeds total debt. Total debt (principal + interest): {} satoshis", total_debt);
            }

            // Check allowance first
            match check_allowance(user).await {
                Ok(allowance) => {
                    if allowance.allowance.0.to_u64().unwrap_or(0) < amount {
                        return format!("Insufficient allowance. Please approve {} satoshis in your Plug wallet first.", amount);
                    }
                }
                Err(e) => return format!("Failed to check allowance: {}", e),
            }

            match transfer_ckbtc_from_user_to_canister(user, amount).await {
                Ok(tx_id) => {
                    let actual_amount = amount - CKBTC_TRANSFER_FEE;
                    
                    if actual_amount >= data.loans {
                        // Full repayment
                        data.loans = 0;
                        data.loan_timestamp = None;
                        format!("Loan fully repaid. Transaction ID: {}", tx_id)
                    } else {
                        // Partial repayment
                        data.loans -= actual_amount;
                        data.loan_timestamp = Some(ic_cdk::api::time()); // Reset interest calculation
                        format!("Partial repayment of {} satoshis. Remaining debt: {} satoshis. Transaction ID: {}", 
                            actual_amount, data.loans, tx_id)
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
async fn stake_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            // Check allowance first
            match check_allowance(user).await {
                Ok(allowance) => {
                    if allowance.allowance.0.to_u64().unwrap_or(0) < amount {
                        return format!("Insufficient allowance. Please approve {} satoshis in your Plug wallet first.", amount);
                    }
                }
                Err(e) => return format!("Failed to check allowance: {}", e),
            }

            match transfer_ckbtc_from_user_to_canister(user, amount).await {
                Ok(tx_id) => {
                    let actual_amount = amount - CKBTC_TRANSFER_FEE;
                    if data.ckbtc_balance < actual_amount {
                        return format!("Insufficient ckBTC balance. You have {} satoshis.", data.ckbtc_balance);
                    }
                    data.ckbtc_balance -= actual_amount;
                    data.staked += actual_amount;
                    data.stake_timestamp = Some(ic_cdk::api::time());
                    format!("Staked {} satoshis (fee: {} satoshis). Earning {}% annual rewards. Transaction ID: {}", 
                        actual_amount, CKBTC_TRANSFER_FEE, (STAKING_RATE * 100.0) as u32, tx_id)
                }
                Err(e) => format!("Staking failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Unstake ckBTC
#[update]
async fn unstake_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.staked < amount {
                return format!("Insufficient staked amount. Staked: {} satoshis", data.staked);
            }
            let rewards = calculate_interest(data.staked, data.stake_timestamp, STAKING_RATE);
            let total_amount = amount + rewards;
            match transfer_ckbtc_from_canister_to_user(user, total_amount).await {
                Ok(tx_id) => {
                    data.staked -= amount;
                    data.ckbtc_balance += amount; // Add back principal to balance
                    if data.staked == 0 {
                        data.stake_timestamp = None;
                    } else {
                        data.stake_timestamp = Some(ic_cdk::api::time());
                    }
                    format!("Unstaked {} satoshis + {} rewards. Transaction ID: {}", amount, rewards, tx_id)
                }
                Err(e) => format!("Unstaking failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Lend ckBTC
#[update]
async fn lend_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            // Check allowance first
            match check_allowance(user).await {
                Ok(allowance) => {
                    if allowance.allowance.0.to_u64().unwrap_or(0) < amount {
                        return format!("Insufficient allowance. Please approve {} satoshis in your Plug wallet first.", amount);
                    }
                }
                Err(e) => return format!("Failed to check allowance: {}", e),
            }

            match transfer_ckbtc_from_user_to_canister(user, amount).await {
                Ok(tx_id) => {
                    let actual_amount = amount - CKBTC_TRANSFER_FEE;
                    if data.ckbtc_balance < actual_amount {
                        return format!("Insufficient ckBTC balance. You have {} satoshis.", data.ckbtc_balance);
                    }
                    data.ckbtc_balance -= actual_amount;
                    data.lent += actual_amount;
                    data.lend_timestamp = Some(ic_cdk::api::time());
                    format!("Lent {} satoshis (fee: {} satoshis). Earning {}% annual rewards. Transaction ID: {}", 
                        actual_amount, CKBTC_TRANSFER_FEE, (LENDING_REWARD * 100.0) as u32, tx_id)
                }
                Err(e) => format!("Lending failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Unlend ckBTC
#[update]
async fn unlend_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.lent < amount {
                return format!("Insufficient lent amount. Lent: {} satoshis", data.lent);
            }
            let rewards = calculate_interest(data.lent, data.lend_timestamp, LENDING_REWARD);
            let total_amount = amount + rewards;
            match transfer_ckbtc_from_canister_to_user(user, total_amount).await {
                Ok(tx_id) => {
                    data.lent -= amount;
                    data.ckbtc_balance += amount; // Add back principal to balance
                    if data.lent == 0 {
                        data.lend_timestamp = None;
                    } else {
                        data.lend_timestamp = Some(ic_cdk::api::time());
                    }
                    format!("Unlent {} satoshis + {} rewards. Transaction ID: {}", amount, rewards, tx_id)
                }
                Err(e) => format!("Unlending failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Yield farm ckBTC
#[update]
async fn yield_farm_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            // Check allowance first
            match check_allowance(user).await {
                Ok(allowance) => {
                    if allowance.allowance.0.to_u64().unwrap_or(0) < amount {
                        return format!("Insufficient allowance. Please approve {} satoshis in your Plug wallet first.", amount);
                    }
                }
                Err(e) => return format!("Failed to check allowance: {}", e),
            }

            match transfer_ckbtc_from_user_to_canister(user, amount).await {
                Ok(tx_id) => {
                    let actual_amount = amount - CKBTC_TRANSFER_FEE;
                    if data.ckbtc_balance < actual_amount {
                        return format!("Insufficient ckBTC balance. You have {} satoshis.", data.ckbtc_balance);
                    }
                    data.ckbtc_balance -= actual_amount;
                    data.farmed += actual_amount;
                    data.farm_timestamp = Some(ic_cdk::api::time());
                    format!("Started yield farming with {} satoshis (fee: {} satoshis). Earning {}% annual rewards. Transaction ID: {}", 
                        actual_amount, CKBTC_TRANSFER_FEE, (YIELD_FARMING_REWARD * 100.0) as u32, tx_id)
                }
                Err(e) => format!("Yield farming failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// Stop yield farming
#[update]
async fn unfarm_ckbtc(amount: u64) -> String {
    if let Err(e) = ensure_not_paused() {
        return e;
    }
    
    if amount == 0 {
        return "Amount must be greater than 0".to_string();
    }

    let user = caller();
    let s = state();
    
    match s.users.get_mut(&user) {
        Some(data) => {
            if data.farmed < amount {
                return format!("Insufficient farmed amount. Farmed: {} satoshis", data.farmed);
            }
            let rewards = calculate_interest(data.farmed, data.farm_timestamp, YIELD_FARMING_REWARD);
            let total_amount = amount + rewards;
            match transfer_ckbtc_from_canister_to_user(user, total_amount).await {
                Ok(tx_id) => {
                    data.farmed -= amount;
                    data.ckbtc_balance += amount; // Add back principal to balance
                    if data.farmed == 0 {
                        data.farm_timestamp = None;
                    } else {
                        data.farm_timestamp = Some(ic_cdk::api::time());
                    }
                    format!("Stopped farming {} satoshis + {} rewards. Transaction ID: {}", amount, rewards, tx_id)
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

            match transfer_ckbtc_from_canister_to_user(user, rewards).await {
                Ok(tx_id) => {
                    data.stake_timestamp = Some(ic_cdk::api::time());
                    data.ckbtc_balance += rewards; // Update balance after claiming
                    format!("Claimed {} satoshis as staking rewards. Transaction ID: {}", rewards, tx_id)
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

            match transfer_ckbtc_from_canister_to_user(user, rewards).await {
                Ok(tx_id) => {
                    data.lend_timestamp = Some(ic_cdk::api::time());
                    data.ckbtc_balance += rewards; // Update balance after claiming
                    format!("Claimed {} satoshis as lending rewards. Transaction ID: {}", rewards, tx_id)
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

            match transfer_ckbtc_from_canister_to_user(user, rewards).await {
                Ok(tx_id) => {
                    data.farm_timestamp = Some(ic_cdk::api::time());
                    data.ckbtc_balance += rewards; // Update balance after claiming
                    format!("Claimed {} satoshis as yield farming rewards. Transaction ID: {}", rewards, tx_id)
                }
                Err(e) => format!("Claim failed: {}", e),
            }
        }
        None => "User not registered".to_string(),
    }
}

// View functions for pending rewards
#[query]
fn get_pending_staking_rewards(user: Option<Principal>) -> u64 {
    let account = user.unwrap_or(caller());
    if let Some(data) = state().users.get(&account) {
        calculate_interest(data.staked, data.stake_timestamp, STAKING_RATE)
    } else {
        0
    }
}

#[query]
fn get_pending_lending_rewards(user: Option<Principal>) -> u64 {
    let account = user.unwrap_or(caller());
    if let Some(data) = state().users.get(&account) {
        calculate_interest(data.lent, data.lend_timestamp, LENDING_REWARD)
    } else {
        0
    }
}

#[query]
fn get_pending_yield_farming_rewards(user: Option<Principal>) -> u64 {
    let account = user.unwrap_or(caller());
    if let Some(data) = state().users.get(&account) {
        calculate_interest(data.farmed, data.farm_timestamp, YIELD_FARMING_REWARD)
    } else {
        0
    }
}

#[query]
fn get_loan_debt(user: Option<Principal>) -> u64 {
    let account = user.unwrap_or(caller());
    if let Some(data) = state().users.get(&account) {
        if data.loans > 0 {
            let interest = calculate_interest(data.loans, data.loan_timestamp, BORROW_RATE);
            data.loans + interest
        } else {
            0
        }
    } else {
        0
    }
}

// Health factor calculation (for liquidation risk)
#[query]
fn get_health_factor(user: Option<Principal>) -> f64 {
    let account = user.unwrap_or(caller());
    if let Some(data) = state().users.get(&account) {
        if data.loans == 0 {
            return f64::INFINITY; // No loans = perfect health
        }

        let total_collateral = data.ckbtc_balance + data.staked + data.lent + data.farmed;
        let total_debt = get_loan_debt(Some(account));
        
        if total_debt == 0 {
            f64::INFINITY
        } else {
            (total_collateral as f64) / (total_debt as f64)
        }
    } else {
        0.0
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

            if total_amount == 0 {
                return "No funds to withdraw".to_string();
            }

            match transfer_ckbtc_from_canister_to_user(user, total_amount).await {
                Ok(tx_id) => {
                    // Reset all user data
                    data.ckbtc_balance = 0;
                    data.staked = 0;
                    data.lent = 0;
                    data.farmed = 0;
                    data.stake_timestamp = None;
                    data.lend_timestamp = None;
                    data.farm_timestamp = None;

                    format!("Emergency withdrawal successful. Total withdrawn: {} satoshis (includes all rewards). Transaction ID: {}", 
                        total_amount, tx_id)
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
        Total Deposits: {} satoshis\n\
        Total Loans: {} satoshis\n\
        Total Staked: {} satoshis\n\
        Total Lent: {} satoshis\n\
        Total Yield Farming: {} satoshis\n\
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
        💡 All amounts are in satoshis (1 BTC = 100,000,000 satoshis)\n\
        💡 Transfer fee: {} satoshis per transaction",
        ic_cdk::id().to_text(),
        (STAKING_RATE * 100.0) as u32,
        (LENDING_REWARD * 100.0) as u32,
        (YIELD_FARMING_REWARD * 100.0) as u32,
        (BORROW_RATE * 100.0) as u32,
        if IS_TESTNET { "Testnet" } else { "Mainnet" },
        CKBTC_TRANSFER_FEE
    )
}