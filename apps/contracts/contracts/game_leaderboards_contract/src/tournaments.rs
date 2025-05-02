use crate::types::Tournament;
use crate::types::LeaderboardEntry;
use soroban_sdk::{Env, String,Vec};


pub fn create_tournament(
    env: &Env,
    tournament_id: String,
    game_id: String,
    start_time: u64,
    end_time: u64,
) {
    let entries = Vec::new(env);

    let tournament = Tournament {
        id: tournament_id.clone(),
        game_id,
        start_time,
        end_time,
        entries,
    };

    env.storage().instance().set(&tournament_id, &tournament);
}

pub fn get_tournament_results(env: &Env, tournament_id: String) -> Vec<LeaderboardEntry> {
    // Retrieve the tournament from storage
    let tournament: Tournament = env.storage().instance().get(&tournament_id).expect("Tournament not found");

    // Return the tournament results
    tournament.entries
}

