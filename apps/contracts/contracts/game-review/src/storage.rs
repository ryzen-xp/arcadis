use crate::types::{DataKey, Review};
use soroban_sdk::{Address, Env};

/// Storage operations for the Game Review contract
pub struct GameReviewStorage;

impl GameReviewStorage {
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

    /// Stores a review in the user-indexed storage
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `user` - Address of the reviewer
    /// * `review` - Review to store
    pub fn set_review(env: &Env, game_id: u32, user: &Address, review: &Review) {
        let key = DataKey::Reviews(game_id, user.clone());
        env.storage().persistent().set(&key, review);
    }

    /// Retrieves a review by game and user
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `user` - Address of the reviewer
    ///
    /// # Returns
    /// * `Review` - The requested review
    pub fn get_review(env: &Env, game_id: u32, user: &Address) -> Review {
        let key = DataKey::Reviews(game_id, user.clone());
        env.storage().persistent().get(&key).unwrap()
    }

    /// Gets the total number of reviews (legacy method)
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    ///
    /// # Returns
    /// * `u32` - Total number of reviews
    pub fn get_total_reviews(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Sets the total number of reviews (legacy method)
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `count` - Count to set
    pub fn set_total_reviews(env: &Env, game_id: u32, count: u32) {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().set(&key, &count);
    }

    /// Gets the sum of all ratings for a game
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    ///
    /// # Returns
    /// * `u32` - Sum of all ratings
    pub fn get_total_ratings(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::TotalRatings(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Sets the sum of all ratings for a game
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `total` - Total sum to set
    pub fn set_total_ratings(env: &Env, game_id: u32, total: u32) {
        let key = DataKey::TotalRatings(game_id);
        env.storage().persistent().set(&key, &total);
    }

    /// Gets the average rating for a game
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    ///
    /// # Returns
    /// * `u32` - Average rating (1-5)
    pub fn get_average_rating(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::AverageRatings(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Sets the average rating for a game
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `rating` - Average rating to set
    pub fn set_average_rating(env: &Env, game_id: u32, rating: u32) {
        let key = DataKey::AverageRatings(game_id);
        env.storage().persistent().set(&key, &rating);
    }

    /// Gets the number of reviews for a game
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    ///
    /// # Returns
    /// * `u32` - Number of reviews
    pub fn get_review_count(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Sets the number of reviews for a game
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `count` - Count to set
    pub fn set_review_count(env: &Env, game_id: u32, count: u32) {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().set(&key, &count);
    }

    /// Gets a review by its ID
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `review_id` - ID of the review
    ///
    /// # Returns
    /// * `Review` - The requested review
    pub fn get_indexed_reviews(env: &Env, game_id: u32, review_id: u32) -> Review {
        let key = DataKey::IndexedReviews(game_id, review_id);
        env.storage().persistent().get(&key).unwrap()
    }

    /// Stores a review by its ID
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `review_id` - ID of the review
    /// * `review` - Review to store
    pub fn set_indexed_reviews(env: &Env, game_id: u32, review_id: u32, review: &Review) {
        let key = DataKey::IndexedReviews(game_id, review_id);
        env.storage().persistent().set(&key, review);
    }
}
