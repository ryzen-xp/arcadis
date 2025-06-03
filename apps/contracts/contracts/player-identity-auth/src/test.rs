#![cfg(test)]
use super::*;
use crate::identity::IdentityManager;
use crate::types::{AccessLevel, Error};
use crate::types::{ADMIN_KEY, PLAYER_COUNTER_KEY};
use soroban_sdk::{
    testutils::Address as _,
    BytesN, Env, String,
};

/// Test Constants
const MAX_USERNAME_LENGTH: u32 = 50;
const MAX_TITLE_LENGTH: u32 = 100;

/// Create a test environment with the necessary contract and client
fn create_test_contracts(env: &Env) -> (Address, PlayerIdentityAuthClient) {
    let contract_id = env.register(PlayerIdentityAuth, ());
    let client = PlayerIdentityAuthClient::new(env, &contract_id);
    (contract_id, client)
}

/// Setup arguments for creating a player profile
fn setup_profile_args(env: &Env) -> (Address, String, BytesN<32>) {
    let player_id = Address::generate(env);
    let username = String::from_str(env, "player1");
    let credentials_hash = BytesN::from_array(env, &[0u8; 32]);
    (player_id, username, credentials_hash)
}

/// Setup arguments for adding an achievement
fn setup_achievement_args(env: &Env) -> (String, String, String) {
    let title = String::from_str(env, "First Win");
    let description = String::from_str(env, "Won the first game");
    let game_id = String::from_str(env, "game123");
    (title, description, game_id)
}

#[test]
fn test_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    env.as_contract(&contract_id, || {
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&ADMIN_KEY)
            .expect("Admin not set");
        let player_counter: u32 = env
            .storage()
            .instance()
            .get(&PLAYER_COUNTER_KEY)
            .expect("Counter not set");
        assert_eq!(stored_admin, admin, "Admin address mismatch");
        assert_eq!(player_counter, 0, "Player counter should be 0");
    });
}

#[test]
fn test_create_profile_success() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);
    assert_eq!(profile_id, 1, "Profile ID should be greater than 0");

    let profile = client.get_player_info(&profile_id);
    assert_eq!(profile.player_id, player_id, "Player ID mismatch");
    assert_eq!(profile.username, username, "Username mismatch");
    assert_eq!(
        profile.credentials_hash, credentials_hash,
        "Credentials hash mismatch"
    );
    assert_eq!(
        profile.access_level,
        AccessLevel::Player,
        "Access level should be Player"
    );
    assert_eq!(
        profile.achievements.len(),
        1,
        "Should have one initial achievement"
    );
    assert_eq!(
        profile.achievements.get(0).unwrap().title,
        String::from_str(&env, "First Achievement"),
        "Initial achievement title mismatch"
    );
}

#[test]
fn test_create_profile_invalid_username() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id, _, credentials_hash) = setup_profile_args(&env);
    let invalid_username = String::from_str(&env, &"a".repeat((MAX_USERNAME_LENGTH + 1) as usize));

    client.initialize(&admin);

    let result = client.try_create_profile(&player_id, &invalid_username, &credentials_hash);
    assert_eq!(
        result,
        Err(Ok(Error::InvalidUsername)),
        "Expected InvalidUsername error"
    );
}

#[test]
fn test_create_profile_username_taken() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id1, username, credentials_hash) = setup_profile_args(&env);
    let player_id2 = Address::generate(&env);

    client.initialize(&admin);

    client.create_profile(&player_id1, &username, &credentials_hash);
    let result = client.try_create_profile(&player_id2, &username, &credentials_hash);
    assert_eq!(
        result,
        Err(Ok(Error::UsernameTaken)),
        "Expected UsernameTaken error"
    );
}

#[test]
fn test_create_profile_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);

    let result = client.try_create_profile(&player_id, &username, &credentials_hash);
    assert_eq!(
        result,
        Err(Ok(Error::NotInitialized)),
        "Expected NotInitialized error"
    );
}

#[test]
fn test_add_achievement_success() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);
    let (title, description, game_id) = setup_achievement_args(&env);

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    let result = client.try_add_achievement(&admin, &profile_id, &title, &description, &game_id);
    assert!(result.is_ok(), "Add achievement failed: {:?}", result);

    env.as_contract(&contract_id, || {
        let profile =
            IdentityManager::get_player_profile(&env, profile_id).expect("Profile not found");
        assert_eq!(
            profile.achievements.len(),
            2,
            "Should have two achievements"
        );
        let achievement = profile.achievements.get(1).unwrap();
        assert_eq!(achievement.title, title, "Achievement title mismatch");
        assert_eq!(
            achievement.description, description,
            "Achievement description mismatch"
        );
        assert_eq!(achievement.game_id, game_id, "Achievement game_id mismatch");
        assert_eq!(
            achievement.timestamp,
            env.ledger().timestamp(),
            "Achievement timestamp mismatch"
        );
    });
}

#[test]
fn test_add_achievement_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);
    let (title, description, game_id) = setup_achievement_args(&env);

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    let result =
        client.try_add_achievement(&non_admin, &profile_id, &title, &description, &game_id);
    assert_eq!(
        result,
        Err(Ok(Error::Unauthorized)),
        "Expected Unauthorized error"
    );
}

#[test]
fn test_add_achievement_invalid_data() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);
    let invalid_title = String::from_str(&env, &"a".repeat((MAX_TITLE_LENGTH + 1) as usize));

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    let result = client.try_add_achievement(
        &admin,
        &profile_id,
        &invalid_title,
        &String::from_str(&env, "desc"),
        &String::from_str(&env, "game123"),
    );
    assert_eq!(
        result,
        Err(Ok(Error::InvalidAchievement)),
        "Expected InvalidAchievement error"
    );

    env.as_contract(&contract_id, || {
        let profile =
            IdentityManager::get_player_profile(&env, profile_id).expect("Profile not found");
        assert_eq!(
            profile.achievements.len(),
            1,
            "Should still have only initial achievement"
        );
    });
}

#[test]
fn test_add_achievement_player_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (title, description, game_id) = setup_achievement_args(&env);

    client.initialize(&admin);

    let non_existent_player_id = 999;
    let result = client.try_add_achievement(
        &admin,
        &non_existent_player_id,
        &title,
        &description,
        &game_id,
    );
    assert_eq!(
        result,
        Err(Ok(Error::PlayerNotFound)),
        "Expected PlayerNotFound error"
    );
}

#[test]
fn test_set_access_level_success() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    client.set_access_level(&admin, &profile_id, &AccessLevel::Moderator);

    env.as_contract(&contract_id, || {
        let profile =
            IdentityManager::get_player_profile(&env, profile_id).expect("Profile not found");
        assert_eq!(
            profile.access_level,
            AccessLevel::Moderator,
            "Access level should be Moderator"
        );
    });
}

#[test]
fn test_set_access_level_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    let result = client.try_set_access_level(&non_admin, &profile_id, &AccessLevel::Moderator);
    assert_eq!(
        result,
        Err(Ok(Error::Unauthorized)),
        "Expected Unauthorized error"
    );

    env.as_contract(&contract_id, || {
        let profile =
            IdentityManager::get_player_profile(&env, profile_id).expect("Profile not found");
        assert_eq!(
            profile.access_level,
            AccessLevel::Player,
            "Access level should remain Player"
        );
    });
}

#[test]
fn test_set_access_level_player_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    let non_existent_player_id = 999;
    let result =
        client.try_set_access_level(&admin, &non_existent_player_id, &AccessLevel::Moderator);
    assert_eq!(
        result,
        Err(Ok(Error::PlayerNotFound)),
        "Expected PlayerNotFound error"
    );
}

#[test]
fn test_verify_credentials_success() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    let result = client.verify_credentials(&profile_id, &credentials_hash);
    assert_eq!(result, true, "Credentials should verify successfully");
}

#[test]
fn test_verify_credentials_incorrect_hash() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);
    let wrong_hash = BytesN::from_array(&env, &[1u8; 32]);

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    let result = client.verify_credentials(&profile_id, &wrong_hash);
    assert_eq!(
        result, false,
        "Credentials should not verify with incorrect hash"
    );
}

#[test]
fn test_verify_credentials_player_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let credentials_hash = BytesN::from_array(&env, &[0u8; 32]);

    client.initialize(&admin);

    let non_existent_player_id = 999;
    let result = client.try_verify_credentials(&non_existent_player_id, &credentials_hash);
    assert_eq!(
        result,
        Err(Ok(Error::PlayerNotFound)),
        "Expected PlayerNotFound error"
    );
}

#[test]
fn test_get_player_info_success() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract_id, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);
    let (player_id, username, credentials_hash) = setup_profile_args(&env);

    client.initialize(&admin);
    let profile_id = client.create_profile(&player_id, &username, &credentials_hash);

    let profile = client.get_player_info(&profile_id);
    assert_eq!(profile.player_id, player_id, "Player ID mismatch");
    assert_eq!(profile.username, username, "Username mismatch");
    assert_eq!(
        profile.credentials_hash, credentials_hash,
        "Credentials hash mismatch"
    );
    assert_eq!(
        profile.access_level,
        AccessLevel::Player,
        "Access level should be Player"
    );
    assert_eq!(
        profile.achievements.len(),
        1,
        "Should have one initial achievement"
    );

    env.as_contract(&contract_id, || {
        let stored_profile =
            IdentityManager::get_player_profile(&env, profile_id).expect("Profile not found");
        assert_eq!(profile, stored_profile, "Stored profile mismatch");
    });
}

#[test]
fn test_get_player_info_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let (_, client) = create_test_contracts(&env);
    let admin = Address::generate(&env);

    client.initialize(&admin);

    let non_existent_player_id = 999;
    let result = client.try_get_player_info(&non_existent_player_id);
    assert_eq!(
        result,
        Err(Ok(Error::PlayerNotFound)),
        "Expected PlayerNotFound error"
    );
}
