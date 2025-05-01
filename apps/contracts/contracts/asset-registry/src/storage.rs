use crate::types::{DataKey, GameAsset};
use soroban_sdk::{Address, BytesN, Env, Vec};

/// Storage operations for the Game Asset Registry contract
pub struct GameAssetStorage;

impl GameAssetStorage {
    /// Sets the contract admin
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `admin` - Address to set as admin
    pub fn set_admin(env: &Env, admin: &Address) {
        let key = DataKey::Admin;
        env.storage().instance().set(&key, admin);
    }

    /// Retrieves the current admin
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    ///
    /// # Returns
    /// * `Address` - Current admin address
    pub fn get_admin(env: &Env) -> Address {
        let key = DataKey::Admin;
        env.storage().instance().get(&key).unwrap()
    }

    /// Stores an asset
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset` - Asset to store
    pub fn set_asset(env: &Env, asset: &GameAsset) {
        let key = DataKey::Asset(asset.id.clone());
        env.storage().persistent().set(&key, asset);
    }

    /// Retrieves an asset by ID
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset_id` - Identifier of the asset
    ///
    /// # Returns
    /// * `GameAsset` - The requested asset
    pub fn get_asset(env: &Env, asset_id: &BytesN<32>) -> GameAsset {
        let key = DataKey::Asset(asset_id.clone());
        env.storage().persistent().get(&key).unwrap()
    }

    /// Checks if an asset exists
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset_id` - Identifier of the asset
    ///
    /// # Returns
    /// * `bool` - True if asset exists, false otherwise
    pub fn has_asset(env: &Env, asset_id: &BytesN<32>) -> bool {
        let key = DataKey::Asset(asset_id.clone());
        env.storage().persistent().has(&key)
    }

    /// Gets all assets owned by a user
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `owner` - Address of the owner
    ///
    /// # Returns
    /// * `Vec<BytesN<32>>` - Vector of asset IDs owned by the user
    pub fn get_user_assets(env: &Env, owner: &Address) -> Vec<BytesN<32>> {
        let key = DataKey::UserAssets(owner.clone());
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(env))
    }

    /// Sets the assets owned by a user
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `owner` - Address of the owner
    /// * `asset_ids` - Vector of asset IDs owned by the user
    pub fn set_user_assets(env: &Env, owner: &Address, asset_ids: &Vec<BytesN<32>>) {
        let key = DataKey::UserAssets(owner.clone());
        env.storage().persistent().set(&key, asset_ids);
    }

    /// Adds an asset to a user's owned assets
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `owner` - Address of the owner
    /// * `asset_id` - Identifier of the asset to add
    pub fn add_user_asset(env: &Env, owner: &Address, asset_id: &BytesN<32>) {
        let mut assets = Self::get_user_assets(env, owner);

        // Only add if not already present
        for existing_id in assets.iter() {
            if &existing_id == asset_id {
                return;
            }
        }

        assets.push_back(asset_id.clone());
        Self::set_user_assets(env, owner, &assets);
    }

    /// Removes an asset from a user's owned assets
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `owner` - Address of the owner
    /// * `asset_id` - Identifier of the asset to remove
    pub fn remove_user_asset(env: &Env, owner: &Address, asset_id: &BytesN<32>) {
        let mut assets = Self::get_user_assets(env, owner);
        let mut index_to_remove = None;

        for (i, existing_id) in assets.iter().enumerate() {
            if &existing_id == asset_id {
                index_to_remove = Some(i);
                break;
            }
        }
        if let Some(index) = index_to_remove {
            assets.remove(index as u32);
            Self::set_user_assets(env, owner, &assets);
        }
    }

    /// Gets all registered asset IDs
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    ///
    /// # Returns
    /// * `Vec<BytesN<32>>` - Vector of all registered asset IDs
    pub fn get_all_asset_ids(env: &Env) -> Vec<BytesN<32>> {
        let key = DataKey::AllAssetIds;
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(env))
    }

    /// Sets all registered asset IDs
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset_ids` - Vector of all registered asset IDs
    pub fn set_all_asset_ids(env: &Env, asset_ids: &Vec<BytesN<32>>) {
        let key = DataKey::AllAssetIds;
        env.storage().persistent().set(&key, asset_ids);
    }

    /// Adds an asset ID to the list of all registered assets
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset_id` - Identifier of the asset to add
    pub fn add_asset_id(env: &Env, asset_id: &BytesN<32>) {
        let mut asset_ids = Self::get_all_asset_ids(env);

        // Only add if not already present
        for existing_id in asset_ids.iter() {
            if &existing_id == asset_id {
                return;
            }
        }

        asset_ids.push_back(asset_id.clone());
        Self::set_all_asset_ids(env, &asset_ids);
    }

    /// Gets the total count of all registered assets
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    ///
    /// # Returns
    /// * `u32` - Total number of registered assets
    pub fn get_asset_count(env: &Env) -> u32 {
        let key = DataKey::AssetCount;
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Sets the total count of all registered assets
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `count` - Count to set
    pub fn set_asset_count(env: &Env, count: u32) {
        let key = DataKey::AssetCount;
        env.storage().persistent().set(&key, &count);
    }

    /// Increments the asset count by 1
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    pub fn increment_asset_count(env: &Env) {
        let count = Self::get_asset_count(env) + 1;
        Self::set_asset_count(env, count);
    }
}
