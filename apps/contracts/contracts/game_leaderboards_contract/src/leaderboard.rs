use crate::types::{LeaderboardEntry, Tournament};
use soroban_sdk::{Env, Vec, String};

pub fn get_leaderboard(env: &Env, tournament_id: String) -> Vec<LeaderboardEntry> {
    // Retrieve the tournament from storage
    let tournament: Tournament = env.storage().instance().get(&tournament_id).expect("Tournament not found");

    // Return the leaderboard entries
    tournament.entries
}

