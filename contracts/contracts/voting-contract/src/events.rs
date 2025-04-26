use soroban_sdk::{contracttype, symbol_short, Address, Symbol};

// Symbol representing GAME events.
pub const GAME: Symbol = symbol_short!("GAME");

// Symbol representing ADD events.
pub const ADD: Symbol = symbol_short!("ADD");

// Symbol representing Vote events.
pub const VOTE: Symbol = symbol_short!("VOTE");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewGameAdded {
    pub game_id: u32,
    pub creator: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteRegistered {
    pub game_id: u32,
    pub voter: Address,
}
