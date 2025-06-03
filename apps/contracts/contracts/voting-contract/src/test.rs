#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String, Vec,
};

use crate::voting::{VotingContract, VotingContractClient};

// Helper function to set up test environment
fn setup_test() -> (Env, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(VotingContract, ());
    let admin = Address::generate(&env);
    env.mock_all_auths();

    (env, contract_id, admin)
}

#[test]
fn test_add_game() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let game_name = String::from_str(&env, "Test Game");

    // Test adding a game
    contract.add_game(&admin, &game_name);
    
    // Verify total games counter
    assert_eq!(contract.get_total_games(), 1);
    
    // Verify game info
    let (id, name, votes, game_creator) = contract.get_game_info(&1);
    assert_eq!(id, 1);
    assert_eq!(name, game_name);
    assert_eq!(votes, 0);
    assert_eq!(game_creator, admin);

    // Test adding multiple games
    let creator2 = Address::generate(&env);
    let game_name2 = String::from_str(&env, "Test Game 2");
    contract.add_game(&creator2, &game_name2);
    
    assert_eq!(contract.get_total_games(), 2);
    
    let (id2, name2, votes2, game_creator2) = contract.get_game_info(&2);
    assert_eq!(id2, 2);
    assert_eq!(name2, game_name2);
    assert_eq!(votes2, 0);
    assert_eq!(game_creator2, creator2);
}

#[test]
#[should_panic(expected = "Error(Contract, #103)")]
fn test_add_game_empty_name() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let empty_name = String::from_str(&env, "");

    contract.add_game(&admin, &empty_name);
}

#[test]
fn test_vote() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let voter = Address::generate(&env);
    let game_name = String::from_str(&env, "Test Game");

    // Add a game
    contract.add_game(&admin, &game_name);
    
    // Test voting
    contract.vote(&voter, &1);
    
    // Verify vote count
    assert_eq!(contract.get_game_votes(&1), 1);
    assert!(contract.has_voted(&voter, &1));

    // Test multiple voters
    let voter2 = Address::generate(&env);
    contract.vote(&voter2, &1);
    
    assert_eq!(contract.get_game_votes(&1), 2);
    assert!(contract.has_voted(&voter2, &1));
}

#[test]
#[should_panic(expected = "Error(Contract, #101)")]
fn test_vote_twice() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let voter = Address::generate(&env);
    let game_name = String::from_str(&env, "Test Game");

    contract.add_game(&admin, &game_name);
    contract.vote(&voter, &1);
    contract.vote(&voter, &1); // Should panic
}

#[test]
#[should_panic(expected = "Error(Contract, #102)")]
fn test_vote_invalid_game() {
    let (env, contract_id, _) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let voter = Address::generate(&env);

    contract.vote(&voter, &999); // Non-existent game ID
}

#[test]
fn test_has_voted() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let voter = Address::generate(&env);
    let non_voter = Address::generate(&env);
    let game_name = String::from_str(&env, "Test Game");

    contract.add_game(&admin, &game_name);
    
    // Before voting
    assert!(!contract.has_voted(&voter, &1));
    assert!(!contract.has_voted(&non_voter, &1));
    
    // After voting
    contract.vote(&voter, &1);
    assert!(contract.has_voted(&voter, &1));
    assert!(!contract.has_voted(&non_voter, &1));
}

#[test]
fn test_get_game_votes() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let game_name = String::from_str(&env, "Test Game");

    contract.add_game(&admin, &game_name);
    
    // Initial votes
    assert_eq!(contract.get_game_votes(&1), 0);
    
    // After one vote
    contract.vote(&voter1, &1);
    assert_eq!(contract.get_game_votes(&1), 1);
    
    // After two votes
    contract.vote(&voter2, &1);
    assert_eq!(contract.get_game_votes(&1), 2);
    
    // Non-existent game
    assert_eq!(contract.get_game_votes(&999), 0);
}

#[test]
fn test_get_total_games() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let creator2 = Address::generate(&env);
    let game_name1 = String::from_str(&env, "Game 1");
    let game_name2 = String::from_str(&env, "Game 2");

    // Initial count
    assert_eq!(contract.get_total_games(), 0);
    
    // After adding one game
    contract.add_game(&admin, &game_name1);
    assert_eq!(contract.get_total_games(), 1);
    
    // After adding second game
    contract.add_game(&creator2, &game_name2);
    assert_eq!(contract.get_total_games(), 2);
}

#[test]
fn test_get_game_info() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let game_name = String::from_str(&env, "Test Game");

    contract.add_game(&admin, &game_name);
    
    let (id, name, votes, game_creator) = contract.get_game_info(&1);
    assert_eq!(id, 1);
    assert_eq!(name, game_name);
    assert_eq!(votes, 0);
    assert_eq!(game_creator, admin);
}

#[test]
#[should_panic]
fn test_get_game_info_invalid_game() {
    let (env, contract_id, _) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    
    contract.get_game_info(&999); // Should panic for non-existent game
}

#[test]
fn test_multiple_games_voting() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let creator2 = Address::generate(&env);
    let voter = Address::generate(&env);
    let game_name1 = String::from_str(&env, "Game 1");
    let game_name2 = String::from_str(&env, "Game 2");

    // Add two games
    contract.add_game(&admin, &game_name1);
    contract.add_game(&creator2, &game_name2);
    
    // Vote on first game
    contract.vote(&voter, &1);
    assert_eq!(contract.get_game_votes(&1), 1);
    assert!(contract.has_voted(&voter, &1));
    assert!(!contract.has_voted(&voter, &2));
    
    // Vote on second game
    contract.vote(&voter, &2);
    assert_eq!(contract.get_game_votes(&2), 1);
    assert!(contract.has_voted(&voter, &2));
}

#[test]
fn test_vote_sequence() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    let game_name = String::from_str(&env, "Test Game");
    
    // Add game
    contract.add_game(&admin, &game_name);
    
    // Create multiple voters
    let mut voters = Vec::new(&env);
    for _ in 0..5 {
        voters.push_back(Address::generate(&env));
    }
    
    // Each voter votes
    for (i, voter) in voters.iter().enumerate() {
        contract.vote(&voter, &1);
        assert_eq!(contract.get_game_votes(&1), (i + 1) as u32);
        assert!(contract.has_voted(&voter, &1));
    }
    
    // Verify final state
    assert_eq!(contract.get_game_votes(&1), 5);
    for voter in voters.iter() {
        assert!(contract.has_voted(&voter, &1));
    }
}

#[test]
fn test_game_creation_sequence() {
    let (env, contract_id, admin) = setup_test();
    let contract = VotingContractClient::new(&env, &contract_id);
    
    // Create multiple games
    for i in 0..5 {
        let game_name = String::from_str(&env, &format_game_name(i + 1));
        contract.add_game(&admin, &game_name);
        
        // Verify each game was created correctly
        let (id, _, votes, creator) = contract.get_game_info(&(i + 1));
        assert_eq!(id, i + 1);
        assert_eq!(votes, 0);
        assert_eq!(creator, admin);
    }
    
    // Verify total games count
    assert_eq!(contract.get_total_games(), 5);
}

// Helper function to format game names
fn format_game_name(num: u32) -> &'static str {
    match num {
        1 => "Game 1",
        2 => "Game 2",
        3 => "Game 3",
        4 => "Game 4",
        5 => "Game 5",
        _ => "Game",
    }
}
