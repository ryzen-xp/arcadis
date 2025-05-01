use soroban_sdk::{contracttype, Address, Bytes};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Review {
    pub id: u32,
    pub reviewer: Address,
    pub rating: u32,
    pub comment: Bytes,
    pub timestamp: u64,
}

/// Data keys for storage operations
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,                    //admin
    ReviewCounts(u32),        //game_id
    TotalRatings(u32),        //game_id
    AverageRatings(u32),      //game_id
    Reviews(u32, Address),    //game_id, reviewer
    IndexedReviews(u32, u32), //game_id, review_id
}
