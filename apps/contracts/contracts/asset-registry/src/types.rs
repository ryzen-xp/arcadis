use soroban_sdk::{contracttype, Address, Bytes, BytesN, Map, Vec};

/// Represents a game asset with metadata and ownership history
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameAsset {
    /// Unique identifier for this asset
    pub id: BytesN<32>,
    /// Current owner of the asset
    pub owner: Address,
    /// Metadata for the asset
    pub metadata: AssetMetadata,
    /// History of ownership transfers
    pub history: Vec<OwnershipRecord>,
}

/// Metadata for a game asset
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetMetadata {
    /// Name of the asset
    pub name: Bytes,
    /// Type of item (e.g., "weapon", "armor", "collectible")
    pub item_type: Bytes,
    /// Additional attributes as key-value pairs
    pub attributes: Map<Bytes, Bytes>,
}

/// Record of ownership transfer
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnershipRecord {
    /// Previous owner of the asset
    pub previous_owner: Address,
    /// Timestamp of the ownership transfer
    pub timestamp: u64,
    /// Reason for the ownership transfer
    pub reason: Bytes,
}

/// Storage keys for the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Key for the admin address
    Admin,
    /// Key for storing assets by ID
    Asset(BytesN<32>),
    /// Key for storing assets owned by a user
    UserAssets(Address),
    /// Key for tracking all registered asset IDs
    AllAssetIds,
    /// Key for tracking total count of all assets
    AssetCount,
}
