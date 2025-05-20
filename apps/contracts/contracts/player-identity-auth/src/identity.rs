extern crate alloc;

use crate::types::{AccessLevel, Achievement, Error, PlayerProfile, ADMIN_KEY, PLAYER_COUNTER_KEY};
use soroban_sdk::{log, symbol_short, vec, Address, BytesN, Env, String, Symbol, Vec};

pub struct IdentityManager;

impl IdentityManager {
    /// Create a new player profile
    pub fn create_profile(
        env: &Env,
        player_id: Address,
        username: String,
        credentials_hash: BytesN<32>,
    ) -> Result<u32, Error> {
        if !env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::NotInitialized);
        }

        player_id.require_auth();

        if username.len() > 50 {
            return Err(Error::InvalidUsername);
        }

        // Check if username is already taken
        let all_profiles = Self::get_all_profiles(env);
        for id in all_profiles.iter() {
            if let Ok(profile) = Self::get_player_profile(env, id) {
                if profile.username == username {
                    return Err(Error::UsernameTaken);
                }
            }
        }

        // Increment player counter
        let mut player_counter: u32 = env
            .storage()
            .instance()
            .get(&PLAYER_COUNTER_KEY)
            .unwrap_or(0);
        player_counter += 1;

        let player_achievement = Achievement {
            title: String::from_str(&env, "First Achievement"),
            description: String::from_str(&env, "Created your first profile!"),
            timestamp: env.ledger().timestamp(),
            game_id: String::from_str(&env, "N/A"),
        };
        let mut achievements = Vec::new(&env);
        achievements.push_back(player_achievement);

        // Create new profile
        let profile = PlayerProfile {
            player_id: player_id.clone(),
            username,
            credentials_hash,
            achievements,
            access_level: AccessLevel::Player,
        };

        // Store profile
        let key = Self::get_profile_key(env, player_counter);
        env.storage().instance().set(&key, &profile);
        env.storage()
            .instance()
            .set(&PLAYER_COUNTER_KEY, &player_counter);

        // Emit event
        env.events().publish(
            (symbol_short!("player"), symbol_short!("created")),
            (player_counter, player_id),
        );

        log!(&env, "Player profile created: {:?}", player_counter);

        Ok(player_counter)
    }

    /// Get player profile by ID
    pub fn get_player_profile(env: &Env, player_id: u32) -> Result<PlayerProfile, Error> {
        let key = Self::get_profile_key(env, player_id);
        env.storage()
            .instance()
            .get(&key)
            .ok_or(Error::PlayerNotFound)
    }

    /// Verify credentials against stored hash
    pub fn verify_credentials(
        env: &Env,
        player_id: u32,
        credentials_hash: BytesN<32>,
    ) -> Result<bool, Error> {
        let profile = Self::get_player_profile(env, player_id)?;
        Ok(profile.credentials_hash == credentials_hash)
    }

    /// Get all player profile IDs
    fn get_all_profiles(env: &Env) -> Vec<u32> {
        let counter: u32 = env
            .storage()
            .instance()
            .get(&PLAYER_COUNTER_KEY)
            .unwrap_or(0);
        let mut profiles = vec![env];
        for id in 1..=counter {
            if env
                .storage()
                .instance()
                .has(&Self::get_profile_key(env, id))
            {
                profiles.push_back(id);
            }
        }
        profiles
    }

    /// Generate storage key for a player profile
    pub fn get_profile_key(env: &Env, player_id: u32) -> Symbol {
        // Directly create the symbol string
        let key_str = alloc::format!("PLAYER_{}", player_id);
        Symbol::new(env, &key_str)
    }

    // / Generate storage key for a given prefix and ID
    // fn get_key_str<T: Into<u32>>(env: &Env, prefix: Bytes, id: T) -> Symbol {
    //     let mut key_bytes = prefix;
    //     let id_bytes = Bytes::from_slice(env, id.into().to_string().as_bytes());
    //     key_bytes.append(&id_bytes);

    //     // Use a different approach to convert Bytes to str
    //     let key_str = alloc::string::String::from_utf8_lossy(key_bytes.to_val());
    //     Symbol::new(env, &key_str)
    // }
}
