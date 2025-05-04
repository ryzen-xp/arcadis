use crate::types::Review;
use soroban_sdk::{Address, Env, Symbol};

/// Contract event emissions
pub struct GameReviewEvents;

impl GameReviewEvents {
    /// Emits event when contract is initialized
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `admin` - Address of the initial admin
    pub fn emit_contract_initialized(env: &Env, admin: &Address) {
        let topics = (Symbol::new(&env, "contract_initialized"), admin);
        env.events().publish(topics, admin);
    }

    /// Emits event when a new review is added
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game being reviewed
    /// * `user` - Address of the reviewer
    /// * `review` - The review that was added
    pub fn emit_review_added(env: &Env, game_id: u32, user: &Address, review: &Review) {
        let topics = (Symbol::new(&env, "new_review_added"), game_id);
        let data = (user.clone(), review.clone());
        env.events().publish(topics, data);
    }

    /// Emits event when a review is deleted
    ///
    /// # Arguments
    /// * `env` - Reference to the contract environment
    /// * `game_id` - Identifier of the game
    /// * `user` - Address of the reviewer whose review was deleted
    pub fn emit_review_deleted(env: &Env, game_id: u32, user: &Address) {
        let topics = (Symbol::new(&env, "review_deleted"), game_id);
        env.events().publish(topics, user.clone());
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
