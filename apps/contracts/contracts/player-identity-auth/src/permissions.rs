use crate::identity::IdentityManager;
use crate::types::{AccessLevel, Error, PlayerProfile, ADMIN_KEY};
use soroban_sdk::{symbol_short, Address, Env};

pub struct PermissionManager;

impl PermissionManager {
    /// Set access level for a player
    pub fn set_access_level(
        env: &Env,
        caller: Address,
        player_id: u32,
        access_level: AccessLevel,
    ) -> Result<(), Error> {
        if !env.storage().instance().has(&ADMIN_KEY) {
            return Err(Error::NotInitialized);
        }

        caller.require_auth();

        // Only admin can set access levels
        if !Self::is_admin(env, &caller) {
            return Err(Error::Unauthorized);
        }

        // Retrieve player profile
        let key = IdentityManager::get_profile_key(env, player_id);
        let mut profile: PlayerProfile = env
            .storage()
            .instance()
            .get(&key)
            .ok_or(Error::PlayerNotFound)?;

        // Update access level
        profile.access_level = access_level.clone();
        env.storage().instance().set(&key, &profile);

        // Emit event
        env.events().publish(
            (symbol_short!("access"), symbol_short!("updated")),
            (player_id, access_level),
        );

        Ok(())
    }

    /// Check if caller is admin
    fn is_admin(env: &Env, caller: &Address) -> bool {
        let admin: Address = env.storage().instance().get(&ADMIN_KEY).unwrap();
        admin == *caller
    }
}
