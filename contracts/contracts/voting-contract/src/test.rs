#![cfg(test)]

use super::voting::{Contract, ContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let _ = ContractClient::new(&env, &contract_id);
}
