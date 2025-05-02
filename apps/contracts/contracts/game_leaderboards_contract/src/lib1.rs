#![no_std]
use core::fmt::Debug;
extern crate soroban_sdk;
use soroban_sdk::{
    contract, contractimpl, TryFromVal,IntoVal, contracttype, panic_with_error, Address, Env, String, Symbol,
    Val,Vec,
};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
struct PlayerScore {
    player_id: Address,
    score: u64,
    timestamp: u64,
    game_id: String,
    tournament_id: Option<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
struct LeaderboardEntry {
    player_id: Address,
    total_score: u64,
    rank: u32,
}
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
struct Tournament {
    id: String,
    game_id: String,
    start_time: u64,
    end_time: u64,
    entries: Vec<LeaderboardEntry>,
}


#[contract]
pub struct GameLeaderboardContract;

#[contractimpl]
impl GameLeaderboardContract {
    //Allow players to submit scores with timestamp verification
    pub fn submit_score(
        env: Env,
        player_id: Address,
        score: u64,
        game_id: String,
        timestamp: u64,
        tournament_id: Option<String>,
    ) {
        // Retrieve the tournament from storage
        let mut tournament: Tournament = env.storage().instance().get(&tournament_id).expect("Tournament not found");

        // Create a new LeaderboardEntry
        let new_entry = LeaderboardEntry {
            player_id,
            total_score: score,
            rank: 0, // Rank will be determined after insertion
        };
        // Insert the new entry into the correct position in the Vec
        let mut entries = tournament.entries.clone();
        let mut inserted = false;
        let mut i = 0;
        for entry in entries.iter() {
            if score > entry.total_score {
                entries.insert(i, new_entry.clone());
                inserted = true;
                break;
            }
            i += 1;
        }
        if !inserted {
            entries.push_back(new_entry.clone());
        }

        // Update the ranks
        let mut rank = 1;
        for mut entry in entries.iter() {
            entry.rank = rank;
            rank += 1;
        }

        // Update the tournament in storage
        tournament.entries = entries;
        env.storage().instance().set(&tournament_id, &tournament);
        
    }
    //get_leaderboard() – Return top players sorted by score
   
    // Set up a new tournament and initialize leaderboard
    pub fn create_tournament(
        env: Env,
        tournament_id: String,
        game_id: String,
        start_time: u64,
        end_time: u64,
    ) {
        let tournament = Tournament {
            id: tournament_id.clone(),
            game_id,
            start_time,
            end_time,
            entries: Vec::new(&env),
        };
        env.storage().instance().set(&tournament_id, &tournament);
    }

    // Retrieve tournament rankings after completion
    

    // Return historical scores and performance per player
     
   

}