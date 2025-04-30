use soroban_sdk::{Address, Env, String};

/// Interface for the Voting contract.
pub trait VotingTrait {
    /// Adds a new game to the voting system.
    fn add_game(env: Env, creator: Address, name: String);

    /// Casts a vote for a game identified by `game_id`.
    fn vote(env: Env, voter: Address, game_id: u32);

    /// Checks if a specific user has already voted for a specific game.
    fn has_voted(env: &Env, user: Address, game_id: u32) -> bool;

    /// Returns the number of votes a game has received.
    fn get_game_votes(env: &Env, game_id: u32) -> u32;

    /// Returns the total number of games registered.
    fn get_total_games(env: &Env) -> u32;

    /// Returns full information about a game: its id, name, vote count, and creator address.
    fn get_game_info(env: &Env, game_id: u32) -> (u32, String, u32, Address);
}
