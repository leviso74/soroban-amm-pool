#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Symbol};

mod swap;
mod lp;

pub use swap::*;
pub use lp::*;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiquidityPool {
    pub token_a: Address,
    pub token_b: Address,
    pub reserve_a: i128,
    pub reserve_b: i128,
    pub lp_token_supply: i128,
    pub fee_bps: u32,
}

#[contracttype]
pub enum DataKey {
    Pool,
    LpToken,
    Balance(Address),
    Allowance(Address, Address),
}

const FEE_BPS: u32 = 30;

#[contract]
pub struct AmmPool;

#[contractimpl]
impl AmmPool {
    pub fn initialize(env: Env, token_a: Address, token_b: Address) {
        assert!(!env.storage().instance().has(&DataKey::Pool), "already initialized");

        let pool = LiquidityPool {
            token_a: token_a.clone(),
            token_b: token_b.clone(),
            reserve_a: 0,
            reserve_b: 0,
            lp_token_supply: 0,
            fee_bps: FEE_BPS,
        };

        env.storage().instance().set(&DataKey::Pool, &pool);
        env.storage().instance().set(&DataKey::LpToken, &Symbol::new(&env, "LP-AMM"));
    }

    pub fn deposit(env: Env, to: Address, amount_a: i128, amount_b: i128, min_lp_out: i128) -> i128 {
        to.require_auth();

        let mut pool: LiquidityPool = env.storage().instance().get(&DataKey::Pool).unwrap();
        assert!(amount_a > 0 && amount_b > 0, "amounts must be positive");

        token::Client::new(&env, &pool.token_a).transfer(&to, &env.current_contract_address(), &amount_a);
        token::Client::new(&env, &pool.token_b).transfer(&to, &env.current_contract_address(), &amount_b);

        let lp_amount = if pool.lp_token_supply == 0 {
            lp::calculate_initial_lp(amount_a, amount_b)
        } else {
            lp::calculate_lp_mint(&pool, amount_a, amount_b)
        };

        assert!(lp_amount >= min_lp_out, "slippage: lp out too low");

        pool.reserve_a += amount_a;
        pool.reserve_b += amount_b;
        pool.lp_token_supply += lp_amount;

        let balance_key = DataKey::Balance(to.clone());
        let existing: i128 = env.storage().instance().get(&balance_key).unwrap_or(0);
        env.storage().instance().set(&balance_key, &(existing + lp_amount));

        env.storage().instance().set(&DataKey::Pool, &pool);

        lp_amount
    }

    pub fn withdraw(env: Env, to: Address, lp_amount: i128, min_a_out: i128, min_b_out: i128) -> (i128, i128) {
        to.require_auth();

        let mut pool: LiquidityPool = env.storage().instance().get(&DataKey::Pool).unwrap();
        assert!(lp_amount > 0, "lp amount must be positive");
        assert!(lp_amount <= pool.lp_token_supply, "insufficient lp supply");

        let balance_key = DataKey::Balance(to.clone());
        let user_balance: i128 = env.storage().instance().get(&balance_key).unwrap_or(0);
        assert!(lp_amount <= user_balance, "insufficient lp balance");

        let (amount_a, amount_b) = lp::calculate_withdraw(&pool, lp_amount);
        assert!(amount_a >= min_a_out && amount_b >= min_b_out, "slippage: withdraw amounts too low");

        pool.reserve_a -= amount_a;
        pool.reserve_b -= amount_b;
        pool.lp_token_supply -= lp_amount;

        env.storage().instance().set(&balance_key, &(user_balance - lp_amount));
        env.storage().instance().set(&DataKey::Pool, &pool);

        token::Client::new(&env, &pool.token_a).transfer(&env.current_contract_address(), &to, &amount_a);
        token::Client::new(&env, &pool.token_b).transfer(&env.current_contract_address(), &to, &amount_b);

        (amount_a, amount_b)
    }

    pub fn swap_exact_a_for_b(env: Env, to: Address, amount_in: i128, min_out: i128) -> i128 {
        to.require_auth();

        let mut pool: LiquidityPool = env.storage().instance().get(&DataKey::Pool).unwrap();
        assert!(amount_in > 0, "amount in must be positive");

        token::Client::new(&env, &pool.token_a).transfer(&to, &env.current_contract_address(), &amount_in);

        let amount_out = swap::calculate_swap_out(pool.reserve_a, pool.reserve_b, amount_in, pool.fee_bps);
        assert!(amount_out >= min_out, "slippage: out too low");
        assert!(amount_out < pool.reserve_b, "insufficient liquidity");

        pool.reserve_a += amount_in;
        pool.reserve_b -= amount_out;

        env.storage().instance().set(&DataKey::Pool, &pool);

        token::Client::new(&env, &pool.token_b).transfer(&env.current_contract_address(), &to, &amount_out);

        amount_out
    }

    pub fn swap_exact_b_for_a(env: Env, to: Address, amount_in: i128, min_out: i128) -> i128 {
        to.require_auth();

        let mut pool: LiquidityPool = env.storage().instance().get(&DataKey::Pool).unwrap();
        assert!(amount_in > 0, "amount in must be positive");

        token::Client::new(&env, &pool.token_b).transfer(&to, &env.current_contract_address(), &amount_in);

        let amount_out = swap::calculate_swap_out(pool.reserve_b, pool.reserve_a, amount_in, pool.fee_bps);
        assert!(amount_out >= min_out, "slippage: out too low");
        assert!(amount_out < pool.reserve_a, "insufficient liquidity");

        pool.reserve_b += amount_in;
        pool.reserve_a -= amount_out;

        env.storage().instance().set(&DataKey::Pool, &pool);

        token::Client::new(&env, &pool.token_a).transfer(&env.current_contract_address(), &to, &amount_out);

        amount_out
    }

    pub fn get_pool(env: Env) -> LiquidityPool {
        env.storage().instance().get(&DataKey::Pool).unwrap()
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(user)).unwrap_or(0)
    }

    pub fn get_lp_token_name(env: Env) -> Symbol {
        env.storage().instance().get(&DataKey::LpToken).unwrap()
    }
}
