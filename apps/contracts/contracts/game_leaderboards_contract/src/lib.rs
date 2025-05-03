#![no_std]
pub mod leaderboard;
pub mod scores;
pub mod tournaments;
pub mod types;

use crate::types::*;
//use crate::types::LeaderboardEntry;



//use core::fmt::Debug;
extern crate soroban_sdk;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec, Symbol };
#[contract]
pub struct GameLeaderboardContract;

#[contractimpl]
impl GameLeaderboardContract {
   
    pub fn submit_score(env: Env, tournament_id: String, player_id: Address, score: u64) {
        let new_entry = scores::submit_score(&env, tournament_id.clone(), player_id, score);
        env.events().publish((Symbol::new(&env, "score_submitted"), tournament_id), new_entry);
    }
     // Get the leaderboard for a tournament
     pub fn get_leaderboard(env: Env, tournament_id: String) -> Vec<LeaderboardEntry> {
        let leaderboard = leaderboard::get_leaderboard(&env, tournament_id.clone());
        env.events().publish((Symbol::new(&env, "leaderboard_retrieved"), tournament_id), leaderboard.clone());
        leaderboard
    }

    // Create a new tournament
    pub fn create_tournament(env: Env, tournament_id: String, game_id: String, start_time: u64, end_time: u64) {
        tournaments::create_tournament(&env, tournament_id.clone(), game_id, start_time, end_time);
        env.events().publish((Symbol::new(&env, "tournament_created"), tournament_id), ());
    }

    // Get the results of a completed tournament
    pub fn get_tournament_results(env: Env, tournament_id: String) -> Vec<LeaderboardEntry> {
        let results = tournaments::get_tournament_results(&env, tournament_id.clone());
        env.events().publish((Symbol::new(&env, "tournament_results_retrieved"), tournament_id), results.clone());
        results
    }
}