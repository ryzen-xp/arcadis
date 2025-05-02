use soroban_sdk::{Address, Env, String};

use crate::types::{LeaderboardEntry, Tournament};


pub fn submit_score(
    env: &Env,
    tournament_id: String,
    player_id: Address,
    score: u64,
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

