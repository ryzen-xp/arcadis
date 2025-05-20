use crate::identity::IdentityManager;
use crate::types::{Achievement, Error, PlayerProfile, ADMIN_KEY};
use soroban_sdk::{symbol_short, Address, Env, String};

pub struct AchievementManager;

impl AchievementManager {
    /// Add an achievement for a player
    pub fn add_achievement(
        env: &Env,
        caller: Address,
        player_id: u32,
        title: String,
        description: String,
        game_id: String,
    ) -> Result<(), Error> {
        if !env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::NotInitialized);
        }

        caller.require_auth();

        // Only admin or game contract (assumed to be caller) can add achievements
        if !Self::is_admin(env, &caller) {
            return Err(Error::Unauthorized);
        }

        if title.len() > 100 || description.len() > 500 || game_id.len() > 50 {
            return Err(Error::InvalidAchievement);
        }

        // Retrieve player profile
        let key = IdentityManager::get_profile_key(env, player_id);
        let mut profile: PlayerProfile = env
            .storage()
            .instance()
            .get(&key)
            .ok_or(Error::PlayerNotFound)?;

        // Create new achievement
        let achievement = Achievement {
            title,
            description,
            timestamp: env.ledger().timestamp(),
            game_id,
        };

        // Add achievement to profile
        profile.achievements.push_back(achievement.clone());
        env.storage().instance().set(&key, &profile);

        // Emit event
        env.events().publish(
            (symbol_short!("achievmnt"), symbol_short!("added")),
            (player_id, achievement.title),
        );

        Ok(())
    }

    /// Check if caller is admin
    fn is_admin(env: &Env, caller: &Address) -> bool {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin == *caller
    }
}
