use soroban_sdk::{Address, Bytes, BytesN, Env, Map, Vec};

use crate::errors::GameAssetError;
use crate::types::{AssetMetadata, GameAsset, OwnershipRecord};

/// Interface for the Game Asset Registry contract.
pub trait GameAssetTrait {
    /// Initialize the contract with an admin
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `admin` - Address to set as the admin
    ///
    /// # Returns
    /// * `Result<(), GameAssetError>` - Ok if successful, Error if already initialized
    fn initialize(env: Env, admin: Address) -> Result<(), GameAssetError>;

    /// Change the admin of the contract
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `new_admin` - Address of the new admin
    ///
    /// # Returns
    /// * `Result<(), GameAssetError>` - Ok if successful, Error if unauthorized
    ///
    /// # Authentication
    /// * Requires authorization from current admin
    fn change_admin(env: Env, new_admin: Address) -> Result<(), GameAssetError>;

    /// Get the current admin address
    ///
    /// # Arguments
    /// * `env` - The contract environment
    ///
    /// # Returns
    /// * `Result<Address, GameAssetError>` - Admin address if initialized, Error otherwise
    fn get_admin(env: Env) -> Result<Address, GameAssetError>;

    /// Register a new asset
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `owner` - Address of the initial owner
    /// * `asset_id` - Unique identifier for the asset
    /// * `name` - Name of the asset
    /// * `item_type` - Type of the item
    /// * `attributes` - Additional attributes as key-value pairs
    ///
    /// # Returns
    /// * `Result<(), GameAssetError>` - Ok if successful, Error if invalid or already exists
    ///
    /// # Authentication
    /// * Requires authorization from admin
    fn register_asset(
        env: Env,
        owner: Address,
        asset_id: BytesN<32>,
        name: Bytes,
        item_type: Bytes,
        attributes: Map<Bytes, Bytes>,
    ) -> Result<(), GameAssetError>;

    /// Transfer ownership of an asset
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `from` - Current owner address
    /// * `to` - New owner address
    /// * `asset_id` - Unique identifier for the asset
    /// * `reason` - Reason for the transfer
    ///
    /// # Returns
    /// * `Result<(), GameAssetError>` - Ok if successful, Error if not found or unauthorized
    ///
    /// # Authentication
    /// * Requires authorization from current owner
    fn transfer_asset(
        env: Env,
        from: Address,
        to: Address,
        asset_id: BytesN<32>,
        reason: Bytes,
    ) -> Result<(), GameAssetError>;

    /// Update metadata for an asset
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `asset_id` - Unique identifier for the asset
    /// * `name` - New name of the asset
    /// * `item_type` - New type of the item
    /// * `attributes` - New attributes
    ///
    /// # Returns
    /// * `Result<(), GameAssetError>` - Ok if successful, Error if not found or unauthorized
    ///
    /// # Authentication
    /// * Requires authorization from admin or current owner
    fn update_metadata(
        env: Env,
        user: Address,
        asset_id: BytesN<32>,
        name: Bytes,
        item_type: Bytes,
        attributes: Map<Bytes, Bytes>,
    ) -> Result<(), GameAssetError>;

    /// Get detailed information about an asset
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `asset_id` - Unique identifier for the asset
    ///
    /// # Returns
    /// * `Result<GameAsset, GameAssetError>` - Asset if found, Error if not found
    fn get_asset_info(env: Env, asset_id: BytesN<32>) -> Result<GameAsset, GameAssetError>;

    /// Get an asset (alias for get_asset_info)
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `asset_id` - Unique identifier for the asset
    ///
    /// # Returns
    /// * `Result<GameAsset, GameAssetError>` - Asset if found, Error if not found
    fn get_asset(env: Env, asset_id: BytesN<32>) -> Result<GameAsset, GameAssetError>;

    /// Get just the metadata for an asset
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `asset_id` - Unique identifier for the asset
    ///
    /// # Returns
    /// * `Result<AssetMetadata, GameAssetError>` - Asset metadata if found, Error if not found
    fn get_asset_metadata(env: Env, asset_id: BytesN<32>) -> Result<AssetMetadata, GameAssetError>;

    /// Get the ownership history of an asset
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `asset_id` - Unique identifier for the asset
    ///
    /// # Returns
    /// * `Result<Vec<OwnershipRecord>, GameAssetError>` - History records if found, Error if not found
    fn get_asset_history(
        env: Env,
        asset_id: BytesN<32>,
    ) -> Result<Vec<OwnershipRecord>, GameAssetError>;

    /// Get all assets owned by a user
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `owner` - Address of the owner
    ///
    /// # Returns
    /// * `Vec<GameAsset>` - Collection of assets, empty if none found
    fn get_user_assets(env: Env, owner: Address) -> Vec<GameAsset>;

    /// Get all registered asset IDs
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `skip` - Number of assets to skip (for pagination)
    /// * `limit` - Maximum number of assets to return
    ///
    /// # Returns
    /// * `Vec<BytesN<32>>` - Collection of asset IDs, empty if none found
    fn get_all_assets(env: Env, skip: u32, limit: u32) -> Vec<BytesN<32>>;

    /// Check if an asset exists
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `asset_id` - Unique identifier for the asset
    ///
    /// # Returns
    /// * `bool` - true if asset exists, false otherwise
    fn asset_exists(env: Env, asset_id: BytesN<32>) -> bool;

    /// Get the total count of all registered assets
    ///
    /// # Arguments
    /// * `env` - The contract environment
    ///
    /// # Returns
    /// * `u32` - Total number of registered assets
    fn get_asset_count(env: Env) -> u32;
}
