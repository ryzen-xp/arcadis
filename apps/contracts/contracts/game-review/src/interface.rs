use soroban_sdk::{Address, Bytes, Env, Vec};

use crate::errors::GameReviewError;
use crate::types::Review;

/// Interface for the Game Review contract.
pub trait GameReviewTrait {
    /// Initialize the contract with an admin
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `admin` - Address to set as the admin
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if successful, Error if already initialized
    fn initialize(env: Env, admin: Address) -> Result<(), GameReviewError>;

    /// Change the admin of the contract
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `new_admin` - Address of the new admin
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if successful, Error if unauthorized
    ///
    /// # Authentication
    /// * Requires authorization from current admin
    fn change_admin(env: Env, new_admin: Address) -> Result<(), GameReviewError>;

    /// Get the current admin address
    ///
    /// # Arguments
    /// * `env` - The contract environment
    ///
    /// # Returns
    /// * `Result<Address, GameReviewError>` - Admin address if initialized, Error otherwise
    fn get_admin(env: Env) -> Result<Address, GameReviewError>;

    /// Add a new review for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `user` - Address of the reviewer
    /// * `game_id` - Unique identifier for the game
    /// * `rating` - Rating value (1-5 stars)
    /// * `comment` - Text content of the review
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if successful, Error if invalid or duplicate
    ///
    /// # Authentication
    /// * Requires authorization from the user
    fn add_review(
        env: Env,
        user: Address,
        game_id: u32,
        rating: u32,
        comment: Bytes,
    ) -> Result<(), GameReviewError>;

    /// Delete a review from the system (admin only)
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    /// * `review_id` - Identifier for the specific review
    ///
    /// # Returns
    /// * `Result<(), GameReviewError>` - Ok if deleted, Error if not found or unauthorized
    ///
    /// # Authentication
    /// * Requires authorization from admin
    fn delete_review(env: Env, game_id: u32, review_id: u32) -> Result<(), GameReviewError>;

    /// Get a specific review by game ID and user
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    /// * `user` - Address of the reviewer
    ///
    /// # Returns
    /// * `Result<Review, GameReviewError>` - Review if found, Error if not found
    fn get_review(env: Env, game_id: u32, user: Address) -> Result<Review, GameReviewError>;

    /// Get a paginated list of reviews for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    /// * `skip` - Number of reviews to skip (for pagination)
    /// * `limit` - Maximum number of reviews to return
    ///
    /// # Returns
    /// * `Vec<Review>` - Collection of reviews, empty if none found
    fn get_reviews(env: Env, game_id: u32, skip: u32, limit: u32) -> Vec<Review>;

    /// Check if a user has already reviewed a specific game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `user` - Address of the potential reviewer
    /// * `game_id` - Unique identifier for the game
    ///
    /// # Returns
    /// * `bool` - true if user has already reviewed, false otherwise
    fn has_reviewed(env: Env, user: Address, game_id: u32) -> bool;

    /// Get the total number of reviews for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    ///
    /// # Returns
    /// * `u32` - Number of reviews (0 if none)
    fn get_game_review_count(env: Env, game_id: u32) -> u32;

    /// Get the average rating for a game
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `game_id` - Unique identifier for the game
    ///
    /// # Returns
    /// * `u32` - Average rating from 1-5 (0 if no reviews)
    fn get_game_rating(env: Env, game_id: u32) -> u32;
}
