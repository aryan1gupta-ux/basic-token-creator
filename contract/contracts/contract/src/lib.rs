#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

const ADMIN: Symbol = symbol_short!("ADMIN");
const BALANCE: Symbol = symbol_short!("BAL");

#[contract]
pub struct BasicToken;

#[contractimpl]
impl BasicToken {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&ADMIN, &admin);
    }

    fn balance_key(user: &Address) -> (Symbol, Address) {
        (BALANCE, user.clone())
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("Invalid amount");
        }

        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let key = Self::balance_key(&to);
        let balance: i128 = env.storage().instance().get(&key).unwrap_or(0);

        env.storage().instance().set(&key, &(balance + amount));
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("Invalid amount");
        }

        from.require_auth();

        let from_key = Self::balance_key(&from);
        let to_key = Self::balance_key(&to);

        let from_balance: i128 = env.storage().instance().get(&from_key).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env.storage().instance().get(&to_key).unwrap_or(0);

        env.storage().instance().set(&from_key, &(from_balance - amount));
        env.storage().instance().set(&to_key, &(to_balance + amount));
    }

    pub fn balance(env: Env, user: Address) -> i128 {
        let key = Self::balance_key(&user);
        env.storage().instance().get(&key).unwrap_or(0)
    }
}