use crate::types::{AssetMetadata, GameAsset, OwnershipRecord};
use soroban_sdk::{Address, BytesN, Env, Symbol};

/// Contract event emissions
pub struct GameAssetEvents;

impl GameAssetEvents {
    /// Emits event when contract is initialized
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `admin` - Address of the initial admin
    pub fn emit_contract_initialized(env: &Env, admin: &Address) {
        let topics = (Symbol::new(&env, "contract_initialized"), admin);
        env.events().publish(topics, admin);
    }

    /// Emits event when a new asset is registered
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset_id` - Identifier of the asset
    /// * `owner` - Address of the owner
    /// * `asset` - The asset that was registered
    pub fn emit_asset_registered(
        env: &Env,
        asset_id: &BytesN<32>,
        owner: &Address,
        asset: &GameAsset,
    ) {
        let topics = (Symbol::new(&env, "asset_registered"), asset_id);
        let data = (owner.clone(), asset.clone());
        env.events().publish(topics, data);
    }

    /// Emits event when an asset is transferred
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset_id` - Identifier of the asset
    /// * `from` - Address of the previous owner
    /// * `to` - Address of the new owner
    /// * `record` - The ownership record of the transfer
    pub fn emit_asset_transferred(
        env: &Env,
        asset_id: &BytesN<32>,
        from: &Address,
        to: &Address,
        record: &OwnershipRecord,
    ) {
        let topics = (Symbol::new(&env, "asset_transferred"), asset_id);
        let data = (from.clone(), to.clone(), record.clone());
        env.events().publish(topics, data);
    }

    /// Emits event when asset metadata is updated
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `asset_id` - Identifier of the asset
    /// * `old_metadata` - Previous metadata
    /// * `new_metadata` - Updated metadata
    pub fn emit_metadata_updated(
        env: &Env,
        asset_id: &BytesN<32>,
        old_metadata: &AssetMetadata,
        new_metadata: &AssetMetadata,
    ) {
        let topics = (Symbol::new(&env, "metadata_updated"), asset_id);
        let data = (old_metadata.clone(), new_metadata.clone());
        env.events().publish(topics, data);
    }

    /// Emits event when admin is changed
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `old_admin` - Address of the previous admin
    /// * `new_admin` - Address of the new admin
    pub fn emit_admin_changed(env: &Env, old_admin: &Address, new_admin: &Address) {
        let topics = (Symbol::new(&env, "admin_changed"),);
        let data = (old_admin, new_admin);
        env.events().publish(topics, data);
    }
}
