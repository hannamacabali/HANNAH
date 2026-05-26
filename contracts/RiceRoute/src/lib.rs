#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, log, Address, Env};

// ✅ StorageKey must be defined before use
#[contracttype]
pub enum StorageKey {
    ContractState,
}

// ✅ @contracttype → #[contracttype]
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteState {
    pub farmer: Address,
    pub coop: Address,
    pub hauler: Address,
    pub amount: i128,
    pub is_funded: bool,
    pub is_completed: bool,
}

#[contract]
pub struct RiceRoute;

// ✅ @contractimpl → #[contractimpl], and impl name matches struct
#[contractimpl]
impl RiceRoute {
    pub fn init_route(env: Env, farmer: Address, coop: Address, hauler: Address, amount: i128) {
        farmer.require_auth();

        let state = RouteState {
            farmer,
            coop,
            hauler,
            amount,
            is_funded: false,
            is_completed: false,
        };

        env.storage().instance().set(&StorageKey::ContractState, &state);
        log!(&env, "Route initialized successfully.");
    }

    pub fn fund_transport(env: Env) {
        let mut state: RouteState = env
            .storage()
            .instance()
            .get(&StorageKey::ContractState)
            .unwrap();
        state.coop.require_auth();

        if state.is_funded {
            panic!("Agreement already funded");
        }

        state.is_funded = true;
        env.storage().instance().set(&StorageKey::ContractState, &state);
        log!(&env, "Transport funded. Mobilization payment triggered.");
    }

    pub fn complete_delivery(env: Env) {
        let mut state: RouteState = env
            .storage()
            .instance()
            .get(&StorageKey::ContractState)
            .unwrap();
        state.farmer.require_auth();

        if !state.is_funded {
            panic!("Cannot complete an unfunded route");
        }
        if state.is_completed {
            panic!("Route already completed");
        }

        state.is_completed = true;
        env.storage().instance().set(&StorageKey::ContractState, &state);
        log!(&env, "Delivery confirmed. Final balances cleared.");
    }

    pub fn get_state(env: Env) -> RouteState {
        env.storage()
            .instance()
            .get(&StorageKey::ContractState)
            .unwrap()
    }
}