#![no_std]

mod errors;
mod events;
mod interface;
mod storage;
mod types;

// #[cfg(test)]
// mod test;

use errors::GameAssetError;
use events::GameAssetEvents;
use interface::GameAssetTrait;
use storage::GameAssetStorage;
use types::{AssetMetadata, GameAsset, OwnershipRecord};

use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env, Map, Vec};

#[contract]
pub struct GameAssetContract;

#[contractimpl]
impl GameAssetTrait for GameAssetContract {
    fn initialize(env: Env, admin: Address) -> Result<(), GameAssetError> {
        // Check for existing initialization
        if env.storage().instance().has(&types::DataKey::Admin) {
            return Err(GameAssetError::AlreadyInitialized);
        }

        // Set the admin
        GameAssetStorage::set_admin(&env, &admin);

        // Emit initialization event
        GameAssetEvents::emit_contract_initialized(&env, &admin);

        Ok(())
    }

    fn change_admin(env: Env, new_admin: Address) -> Result<(), GameAssetError> {
        // Get the current admin
        let admin = GameAssetStorage::get_admin(&env);

        // Require authorization from current admin
        admin.require_auth();

        // Set the new admin
        GameAssetStorage::set_admin(&env, &new_admin);

        // Emit admin changed event
        GameAssetEvents::emit_admin_changed(&env, &admin, &new_admin);

        Ok(())
    }

    fn get_admin(env: Env) -> Result<Address, GameAssetError> {
        if !env.storage().instance().has(&types::DataKey::Admin) {
            return Err(GameAssetError::AlreadyInitialized);
        }

        Ok(GameAssetStorage::get_admin(&env))
    }

    fn register_asset(
        env: Env,
        owner: Address,
        asset_id: BytesN<32>,
        name: Bytes,
        item_type: Bytes,
        attributes: Map<Bytes, Bytes>,
    ) -> Result<(), GameAssetError> {
        // Get and authorize admin
        let admin = GameAssetStorage::get_admin(&env);
        admin.require_auth();

        // Validate input
        if name.len() == 0 || item_type.len() == 0 {
            return Err(GameAssetError::InvalidInput);
        }

        // Check if asset already exists
        if GameAssetStorage::has_asset(&env, &asset_id) {
            return Err(GameAssetError::AssetAlreadyRegistered);
        }

        // Create metadata
        let metadata = AssetMetadata {
            name,
            item_type,
            attributes,
        };

        // Create empty history vector
        let history = Vec::new(&env);

        // Create the asset
        let asset = GameAsset {
            id: asset_id.clone(),
            owner: owner.clone(),
            metadata,
            history,
        };

        // Store the asset
        GameAssetStorage::set_asset(&env, &asset);

        // Add to user's assets
        GameAssetStorage::add_user_asset(&env, &owner, &asset_id);

        // Add to global asset registry
        GameAssetStorage::add_asset_id(&env, &asset_id);

        // Increment asset count
        GameAssetStorage::increment_asset_count(&env);

        // Emit event
        GameAssetEvents::emit_asset_registered(&env, &asset_id, &owner, &asset);

        Ok(())
    }

    fn transfer_asset(
        env: Env,
        from: Address,
        to: Address,
        asset_id: BytesN<32>,
        reason: Bytes,
    ) -> Result<(), GameAssetError> {
        // Require authorization from current owner
        from.require_auth();

        // Check if asset exists
        if !GameAssetStorage::has_asset(&env, &asset_id) {
            return Err(GameAssetError::AssetNotFound);
        }

        // Get the asset
        let mut asset = GameAssetStorage::get_asset(&env, &asset_id);

        // Verify current owner
        if asset.owner != from {
            return Err(GameAssetError::Unauthorized);
        }

        // Create ownership record
        let record = OwnershipRecord {
            previous_owner: from.clone(),
            timestamp: env.ledger().timestamp(),
            reason: reason.clone(),
        };

        // Add to history
        asset.history.push_back(record.clone());

        // Update owner
        asset.owner = to.clone();

        // Update asset in storage
        GameAssetStorage::set_asset(&env, &asset);

        // Remove from previous owner's assets
        GameAssetStorage::remove_user_asset(&env, &from, &asset_id);

        // Add to new owner's assets
        GameAssetStorage::add_user_asset(&env, &to, &asset_id);

        // Emit transfer event
        GameAssetEvents::emit_asset_transferred(&env, &asset_id, &from, &to, &record);

        Ok(())
    }

    fn update_metadata(
        env: Env,
        user: Address,
        asset_id: BytesN<32>,
        name: Bytes,
        item_type: Bytes,
        attributes: Map<Bytes, Bytes>,
    ) -> Result<(), GameAssetError> {
        // Check if asset exists
        if !GameAssetStorage::has_asset(&env, &asset_id) {
            return Err(GameAssetError::AssetNotFound);
        }

        // Get the asset
        let mut asset = GameAssetStorage::get_asset(&env, &asset_id);

        // Get the admin
        let admin = GameAssetStorage::get_admin(&env);

        // Require authorization from either admin or owner

        // Do we want to allow the admin to update the metadata of any asset?
        if (admin == user) || (asset.owner == user) {
            user.require_auth();
        } else {
            return Err(GameAssetError::Unauthorized);
        }

        // Store original metadata for event
        let old_metadata = asset.metadata.clone();

        // Create new metadata
        let new_metadata = AssetMetadata {
            name,
            item_type,
            attributes,
        };

        // Update metadata
        asset.metadata = new_metadata.clone();

        // Update asset in storage
        GameAssetStorage::set_asset(&env, &asset);

        // Emit metadata updated event
        GameAssetEvents::emit_metadata_updated(&env, &asset_id, &old_metadata, &new_metadata);

        Ok(())
    }

    fn get_asset_info(env: Env, asset_id: BytesN<32>) -> Result<GameAsset, GameAssetError> {
        // Check if asset exists
        if !GameAssetStorage::has_asset(&env, &asset_id) {
            return Err(GameAssetError::AssetNotFound);
        }

        // Return the asset
        Ok(GameAssetStorage::get_asset(&env, &asset_id))
    }

    fn get_asset(env: Env, asset_id: BytesN<32>) -> Result<GameAsset, GameAssetError> {
        Self::get_asset_info(env, asset_id)
    }

    fn get_asset_metadata(env: Env, asset_id: BytesN<32>) -> Result<AssetMetadata, GameAssetError> {
        // Check if asset exists
        if !GameAssetStorage::has_asset(&env, &asset_id) {
            return Err(GameAssetError::AssetNotFound);
        }

        // Get the asset and return just the metadata component
        let asset = GameAssetStorage::get_asset(&env, &asset_id);
        Ok(asset.metadata)
    }

    fn get_asset_history(
        env: Env,
        asset_id: BytesN<32>,
    ) -> Result<Vec<OwnershipRecord>, GameAssetError> {
        // Check if asset exists
        if !GameAssetStorage::has_asset(&env, &asset_id) {
            return Err(GameAssetError::AssetNotFound);
        }

        // Get the asset
        let asset = GameAssetStorage::get_asset(&env, &asset_id);

        // Return the history
        Ok(asset.history)
    }

    fn get_user_assets(env: Env, owner: Address) -> Vec<GameAsset> {
        // Get the asset IDs for this user
        let asset_ids = GameAssetStorage::get_user_assets(&env, &owner);

        // Create a vector to hold the assets
        let mut assets = Vec::new(&env);

        // Fetch each asset
        for id in asset_ids.iter() {
            if GameAssetStorage::has_asset(&env, &id) {
                let asset = GameAssetStorage::get_asset(&env, &id);
                assets.push_back(asset);
            }
        }

        assets
    }

    fn get_all_assets(env: Env, skip: u32, limit: u32) -> Vec<BytesN<32>> {
        // Get all asset IDs
        let all_ids = GameAssetStorage::get_all_asset_ids(&env);

        // Create a new vector for the pagination
        let mut result = Vec::new(&env);

        // Determine start and end indices
        let start = skip;
        let end = if (skip + limit) > all_ids.len() {
            all_ids.len()
        } else {
            skip + limit
        };

        // Skip and limit
        if start < all_ids.len() {
            for i in start..end {
                result.push_back(all_ids.get(i).unwrap());
            }
        }

        result
    }

    fn asset_exists(env: Env, asset_id: BytesN<32>) -> bool {
        GameAssetStorage::has_asset(&env, &asset_id)
    }

    fn get_asset_count(env: Env) -> u32 {
        GameAssetStorage::get_asset_count(&env)
    }
}
