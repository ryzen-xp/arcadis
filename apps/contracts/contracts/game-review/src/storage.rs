use crate::types::{DataKey, Review};
use soroban_sdk::{Address, Env};

pub struct GameReviewStorage;

impl GameReviewStorage {
    pub fn set_admin(env: &Env, admin: &Address) {
        let key = DataKey::Admin;
        env.storage().instance().set(&key, admin);
    }

    pub fn get_admin(env: &Env) -> Address {
        let key = DataKey::Admin;
        env.storage().instance().get(&key).unwrap()
    }

    pub fn set_review(env: &Env, game_id: u32, user: &Address, review: &Review) {
        let key = DataKey::Reviews(game_id, user.clone());
        env.storage().persistent().set(&key, review);
    }

    pub fn get_review(env: &Env, game_id: u32, user: &Address) -> Review {
        let key = DataKey::Reviews(game_id, user.clone());
        env.storage().persistent().get(&key).unwrap()
    }

    pub fn get_total_reviews(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    pub fn set_total_reviews(env: &Env, game_id: u32, count: u32) {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().set(&key, &count);
    }

    pub fn get_total_ratings(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::TotalRatings(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    pub fn set_total_ratings(env: &Env, game_id: u32, total: u32) {
        let key = DataKey::TotalRatings(game_id);
        env.storage().persistent().set(&key, &total);
    }

    pub fn get_average_rating(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::AverageRatings(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    pub fn set_average_rating(env: &Env, game_id: u32, rating: u32) {
        let key = DataKey::AverageRatings(game_id);
        env.storage().persistent().set(&key, &rating);
    }

    pub fn get_review_count(env: &Env, game_id: u32) -> u32 {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    pub fn set_review_count(env: &Env, game_id: u32, count: u32) {
        let key = DataKey::ReviewCounts(game_id);
        env.storage().persistent().set(&key, &count);
    }

    pub fn get_indexed_reviews(env: &Env, game_id: u32, review_id: u32) -> Review {
        let key = DataKey::IndexedReviews(game_id, review_id);
        env.storage().persistent().get(&key).unwrap()
    }

    pub fn set_indexed_reviews(env: &Env, game_id: u32, review_id: u32, review: &Review) {
        let key = DataKey::IndexedReviews(game_id, review_id);
        env.storage().persistent().set(&key, review);
    }
}
