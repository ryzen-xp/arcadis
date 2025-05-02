use soroban_sdk::{contracttype,Address,String, Vec};

#[derive(Clone)]
#[contracttype]
pub struct PlayerScore {
    pub player_id: Address,
    pub score: u64,
    pub timestamp: u64,
    pub game_id: String,
    pub tournament_id: Option<String>,
}
#[derive(Clone)]
#[contracttype]
pub struct LeaderboardEntry {
    pub player_id: Address,
    pub total_score: u64,
    pub rank: u32,
}
#[derive(Clone)]
#[contracttype]
pub struct Tournament {
    pub id: String,
    pub game_id: String,
    pub start_time: u64,
    pub end_time: u64,
    pub entries: Vec<LeaderboardEntry>, // Vec to store leaderboard entries
}
