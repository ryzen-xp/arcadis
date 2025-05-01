use soroban_sdk::{Address, Bytes, Env, Vec};

use crate::errors::GameReviewError;
use crate::types::Review;

/// Interface for the Game Review contract.
pub trait GameReviewTrait {
    /// Initialize the contract with an admin
    fn initialize(env: Env, admin: Address) -> Result<(), GameReviewError>;

    /// Change the admin of the contract
    fn change_admin(env: Env, new_admin: Address) -> Result<(), GameReviewError>;

    /// Get the current admin
    fn get_admin(env: Env) -> Result<Address, GameReviewError>;

    /// Adds a new game to the voting system.
    fn add_review(
        env: Env,
        user: Address,
        game_id: u32,
        rating: u32,
        comment: Bytes,
    ) -> Result<(), GameReviewError>;

    /// Admin function to delete a review
    fn delete_review(env: Env, game_id: u32, review_id: u32) -> Result<(), GameReviewError>;

    fn get_review(env: Env, game_id: u32, user: Address) -> Result<Review, GameReviewError>;

    fn get_reviews(env: Env, game_id: u32, skip: u32, limit: u32) -> Vec<Review>;

    fn has_reviewed(env: Env, user: Address, game_id: u32) -> bool;

    fn get_game_review_count(env: Env, game_id: u32) -> u32;

    fn get_game_rating(env: Env, game_id: u32) -> u32;
}
