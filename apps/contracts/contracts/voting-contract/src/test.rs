#![cfg(test)]

use super::voting::{VotingContract, VotingContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(VotingContract, ());
    let _ = VotingContractClient::new(&env, &contract_id);
}
