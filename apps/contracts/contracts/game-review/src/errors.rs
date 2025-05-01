use soroban_sdk::{contracterror, ConversionError};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum GameReviewError {
    // Contract has already been initialized
    AlreadyInitialized = 1,
    // User is not the admin
    Unauthorized = 2,
    // Invalid input provided
    InvalidInput = 3,
    // Review not found
    ReviewNotFound = 4,
    // User has already reviewed this game
    UserHasReviewed = 5,
}

impl From<ConversionError> for GameReviewError {
    fn from(_: ConversionError) -> Self {
        GameReviewError::InvalidInput
    }
}
