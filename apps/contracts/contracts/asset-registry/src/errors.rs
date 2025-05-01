use soroban_sdk::{contracterror, ConversionError};

/// Errors that can occur during contract execution
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GameAssetError {
    /// Contract has already been initialized
    AlreadyInitialized = 1,
    /// User does not have permission for this action
    Unauthorized = 2,
    /// Input parameters are invalid or out of allowed range
    InvalidInput = 3,
    /// The requested asset does not exist
    AssetNotFound = 4,
    /// The asset ID already exists
    AssetAlreadyRegistered = 5,
    /// The attempted operation is not allowed
    OperationNotAllowed = 6,
}

/// Implementation to convert ConversionError to GameAssetError
impl From<ConversionError> for GameAssetError {
    fn from(_: ConversionError) -> Self {
        GameAssetError::InvalidInput
    }
}
