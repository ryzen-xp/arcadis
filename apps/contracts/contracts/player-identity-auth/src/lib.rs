#![no_std]

extern crate alloc;

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};
use types::{AccessLevel, PlayerProfile, ADMIN_KEY, PLAYER_COUNTER_KEY};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod achievements;
mod identity;
mod permissions;
mod test;
mod types;

#[contract]
pub struct PlayerIdentityAuth;

#[contractimpl]
impl PlayerIdentityAuth {
    /// Initialize the contract with an admin address
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&ADMIN_KEY, &admin);
        env.storage().instance().set(&PLAYER_COUNTER_KEY, &0u32);
    }

    /// Create a new player profile
    pub fn create_profile(
        env: Env,
        player_id: Address,
        username: String,
        credentials_hash: BytesN<32>,
    ) -> Result<u32, types::Error> {
        identity::IdentityManager::create_profile(&env, player_id, username, credentials_hash)
    }

    /// Add an achievement for a player
    pub fn add_achievement(
        env: Env,
        caller: Address,
        player_id: u32,
        title: String,
        description: String,
        game_id: String,
    ) -> Result<(), types::Error> {
        achievements::AchievementManager::add_achievement(
            &env,
            caller,
            player_id,
            title,
            description,
            game_id,
        )
    }

    /// Set the access level for a player
    pub fn set_access_level(
        env: Env,
        caller: Address,
        player_id: u32,
        access_level: AccessLevel,
    ) -> Result<(), types::Error> {
        permissions::PermissionManager::set_access_level(&env, caller, player_id, access_level)
    }

    /// Get player profile information
    pub fn get_player_info(env: Env, player_id: u32) -> Result<PlayerProfile, types::Error> {
        identity::IdentityManager::get_player_profile(&env, player_id)
    }

    /// Verify player credentials
    pub fn verify_credentials(
        env: Env,
        player_id: u32,
        credentials_hash: BytesN<32>,
    ) -> Result<bool, types::Error> {
        identity::IdentityManager::verify_credentials(&env, player_id, credentials_hash)
    }
}
