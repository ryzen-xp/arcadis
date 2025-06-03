use soroban_sdk::{
    contracterror, contracttype, symbol_short, Address, BytesN, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct PlayerProfile {
    pub player_id: Address,
    pub username: String,
    pub credentials_hash: BytesN<32>,
    pub achievements: Vec<Achievement>,
    pub access_level: AccessLevel,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Achievement {
    pub title: String,
    pub description: String,
    pub timestamp: u64,
    pub game_id: String,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum AccessLevel {
    Player,
    Moderator,
    Admin,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    PlayerNotFound = 2,
    InvalidUsername = 3,
    UsernameTaken = 4,
    InvalidAchievement = 5,
    Unauthorized = 6,
}

pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
pub const PLAYER_COUNTER_KEY: Symbol = symbol_short!("PLYR_CNT");
