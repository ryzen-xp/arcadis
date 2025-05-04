use soroban_sdk::{contracterror, ConversionError};

/// Errors that can occur during contract execution
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GameReviewError {
    /// Contract has already been initialized
    AlreadyInitialized = 1,
    /// User does not have permission for this action
    Unauthorized = 2,
    /// Input parameters are invalid or out of allowed range
    InvalidInput = 3,
    /// The requested review does not exist
    ReviewNotFound = 4,
    /// User has already submitted a review for this game
    UserHasReviewed = 5,
}

/// Implementation to convert ConversionError to GameReviewError
impl From<ConversionError> for GameReviewError {
    fn from(_: ConversionError) -> Self {
        GameReviewError::InvalidInput
    }
}
