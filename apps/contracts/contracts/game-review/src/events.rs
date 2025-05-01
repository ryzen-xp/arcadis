use crate::types::Review;
use soroban_sdk::{Address, Env, Symbol};

pub struct GameReviewEvents;

impl GameReviewEvents {
    pub fn emit_contract_initialized(env: &Env, admin: &Address) {
        let topics = (Symbol::new(&env, "contract_initialized"), admin);
        env.events().publish(topics, admin);
    }

    pub fn emit_review_added(env: &Env, game_id: u32, user: &Address, review: &Review) {
        let topics = (Symbol::new(&env, "new_review_added"), game_id);
        let data = (user.clone(), review.clone());
        env.events().publish(topics, data);
    }

    pub fn emit_review_deleted(env: &Env, game_id: u32, user: &Address) {
        let topics = (Symbol::new(&env, "review_deleted"), game_id);
        env.events().publish(topics, user.clone());
    }

    pub fn emit_admin_changed(env: &Env, old_admin: &Address, new_admin: &Address) {
        let topics = (Symbol::new(&env, "admin_changed"),);
        let data = (old_admin, new_admin);
        env.events().publish(topics, data);
    }
}
