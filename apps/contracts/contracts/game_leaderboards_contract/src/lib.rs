#![no_std]
pub mod leaderboard;
pub mod scores;
pub mod tournaments;
pub mod types;

use crate::types::*;
//use crate::types::LeaderboardEntry;



//use core::fmt::Debug;
extern crate soroban_sdk;
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};
#[contract]
pub struct GameLeaderboardContract;

#[contractimpl]
impl GameLeaderboardContract {
    pub fn submit_score( env: &Env, tournament_id: String, player_id: Address, score: u64){
        scores::submit_score(&env, tournament_id, player_id, score);
    }

    pub fn get_leaderboard(env: &Env, tournament_id: String) -> Vec<LeaderboardEntry>  {
        leaderboard::get_leaderboard(&env, tournament_id)
    }

    pub fn create_tournament(env: &Env, tournament_id: String, game_id: String, start_time: u64, end_time: u64) {
        tournaments::create_tournament(&env, tournament_id, game_id, start_time, end_time);
    }

    pub fn get_tournament_results(env: &Env, tournament_id: String) -> Vec<LeaderboardEntry> {
        tournaments::get_tournament_results(&env, tournament_id)
    }

}