use soroban_sdk::{contracttype, Address, Bytes};

/// Represents a single game review
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    /// Unique identifier for this review
    pub id: u32,
    /// Address of the user who submitted the review
    pub reviewer: Address,
    /// Rating value from 1-5 stars
    pub rating: u32,
    /// Text content of the review
    pub comment: Bytes,
    /// Ledger timestamp when review was created
    pub timestamp: u64,
}

/// Storage keys for the contract
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Key for the admin address
    Admin,
    /// Key for storing review counts per game
    ReviewCounts(u32),
    /// Key for storing sum of all ratings for a game
    TotalRatings(u32),
    /// Key for storing average rating for a game
    AverageRatings(u32),
    /// Key for accessing reviews by game and reviewer
    Reviews(u32, Address),
    /// Key for accessing reviews by game and review id
    IndexedReviews(u32, u32),
}
