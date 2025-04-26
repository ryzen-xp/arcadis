use crate::errors::Error;
use crate::events::*;
use crate::traits::VotingTrait;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, Env, IntoVal, String, Val,
};

/// Main contract definition.
#[contract]
pub struct Contract;

/// Struct representing a Game.
#[contracttype]
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Game {
    pub id: u32,          // Internal ID assigned to the game
    pub name: String,     // Name of the game
    pub votes: u32,       // Number of votes (likes) the game has received
    pub creator: Address, // Address of the user who created the game
}

/// Enum representing keys used to store contract data in Soroban storage.
#[contracttype]
#[derive(Clone)]
enum DataKey {
    TotalGames,             // Key for storing total number of games
    Game(u32),              // Key for storing a specific Game by its internal ID
    HasVoted(Address, u32), // Key for storing if a user has voted for a specific game
}

/// Contract implementation of the VotingTrait
#[contractimpl]
impl VotingTrait for Contract {
    /// Adds a new game to the system.
    fn add_game(env: Env, creator: Address, name: String) {
        creator.require_auth(); // Ensure caller is authenticated

        if name.is_empty() {
            panic_with_error!(&env, Error::GameNameCannotBeEmpty); // Validate name is not empty
        }

        let total_games = Self::get_total_games(&env); // Read current total games count

        let game_id = total_games + 1; // Generate new game ID by incrementing

        // Build the new game struct
        let game = Game {
            id: game_id,
            name,
            votes: 0,
            creator: creator.clone(),
        };

        // Save the new game into storage
        Self::_save_game(&env, game_id.clone(), &game);

        // Update the total games count
        Self::_save_total_games(&env, &game_id);

        // Emit event signaling new game added
        env.events()
            .publish((GAME, ADD), NewGameAdded { game_id, creator });
    }

    /// Allows a user to vote for a specific game.
    fn vote(env: Env, voter: Address, game_id: u32) {
        voter.require_auth(); // Ensure voter is authenticated

        let game = Self::_get_game(&env, game_id); // Fetch game by ID

        if game.is_none() {
            panic_with_error!(&env, Error::GameNotFound); // Game must exist
        }

        if Self::has_voted(&env, voter.clone(), game_id.clone()) {
            panic_with_error!(&env, Error::UserHasVoted); // User must not have voted before
        }

        let mut game = game.unwrap(); // Unwrap safely since we checked it's Some

        game.votes += 1; // Increment the game's vote count

        Self::_save_game(&env, game_id.clone(), &game); // Save updated game

        Self::_register_vote(&env, voter.clone(), game_id.clone()); // Mark user as voted

        // Emit event signaling vote registration
        env.events()
            .publish((GAME, VOTE), VoteRegistered { game_id, voter });
    }

    /// Checks if a user has already voted for a game.
    fn has_voted(env: &Env, user: Address, game_id: u32) -> bool {
        env.storage()
            .instance()
            .get::<Val, bool>(&DataKey::HasVoted(user, game_id).into_val(env))
            .unwrap_or(false) // Default to false if no record found
    }

    /// Returns the number of votes for a game.
    fn get_game_votes(env: &Env, game_id: u32) -> u32 {
        let game = Self::_get_game(&env, game_id);
        if let Some(game_info) = game {
            game_info.votes
        } else {
            0 // If no game found, return 0 votes
        }
    }

    /// Returns the total number of games added.
    fn get_total_games(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get::<Val, u32>(&DataKey::TotalGames.into_val(env))
            .unwrap_or(0) // Default to 0 if no games exist
    }

    /// Returns full game info (id, name, votes, creator) for a given game ID.
    fn get_game_info(env: &Env, game_id: u32) -> (u32, String, u32, Address) {
        let game = Self::_get_game(&env, game_id).unwrap();
        (game.id, game.name, game.votes, game.creator)
    }
}

impl Contract {
    /// Internal helper to fetch a game from storage.
    fn _get_game(env: &Env, game_id: u32) -> Option<Game> {
        env.storage()
            .instance()
            .get::<Val, Game>(&DataKey::Game(game_id).into_val(env))
    }

    /// Internal helper to save a game to storage.
    fn _save_game(env: &Env, game_id: u32, game: &Game) {
        env.storage()
            .instance()
            .set::<Val, Game>(&DataKey::Game(game_id).into_val(env), game);
    }

    /// Internal helper to save total number of games to storage.
    fn _save_total_games(env: &Env, total_games: &u32) {
        env.storage()
            .instance()
            .set::<Val, u32>(&DataKey::TotalGames.into_val(env), total_games);
    }

    /// Internal helper to register that a user has voted for a game.
    fn _register_vote(env: &Env, voter: Address, game_id: u32) {
        env.storage()
            .instance()
            .set::<Val, bool>(&DataKey::HasVoted(voter, game_id).into_val(env), &true);
    }
}
