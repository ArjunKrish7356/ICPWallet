use ic_cdk::export::{candid::{CandidType, Deserialize},Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(CandidType, Deserialize)]
struct TokenWallet{
    balances: HashMap<Principal, u64>,
    owner: Principal,
}

impl TokenWallet {
    fn new() -> Self{
        Self {
            balances: HashMap::new(),
            owner: Principal::anonymous(),
        }
    }

    fn init(&mut self, owner: Principal){
        self.owner = owner;
    }

    fn send_tokens(&mut self, from: Principal, to: Principal, amount: u64) -> Result<(), String>{
        let from_balance = self.balances.get(&from).copied().unwrap_or(0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }
        
        *self.balances.entry(from).or_insert(0) -= amount;
        *self.balances.entry(to).or_insert(0) += amount;
        
        Ok(())
    }

    fn receive_tokens(&mut self, to: Principal, amount: u64) -> Result<(), String> {
        *self.balances.entry(to).or_insert(0) += amount;
        Ok(())
    }

    fn get_balance(&self, of: Principal) -> u64{
        *self.balances.get(&of).unwrap_or(&0)
    }

    fn get_owner(&self) -> Principal {
        self.owner
    }
}

thread_local! {
    static TOKEN_WALLET: RefCell<TokenWallet> = RefCell::new(TokenWallet::new());
}

#[init]
fn init() {
    TOKEN_WALLET.with(|wallet| {
        wallet.borrow_mut().init(ic_cdk::caller());
    });
}

#[update]
fn send_tokens(to: Principal, amount: u64) -> Result<(), String>{
    TOKEN_WALLET.with(|wallet| {
        wallet.borrow_mut().send_tokens(ic_cdk::caller(), to, amount)
    })
}

#[update]
fn receive_tokens(_from: Principal, amount: u64) -> Result<(), String> {
    TOKEN_WALLET.with(|wallet| {
        wallet.borrow_mut().receive_tokens(ic_cdk::caller(), amount)
    })
}

#[query]
fn get_balance(of: Principal) -> u64{
    TOKEN_WALLET.with(|wallet| wallet.borrow().get_balance(of))
}

#[query]
fn get_owner() -> Principal{
    TOKEN_WALLET.with(|wallet| wallet.borrow().get_owner())
}


#[cfg(test)]
mod tests;