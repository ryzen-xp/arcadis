#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, token, Address, BytesN, Env, IntoVal,
    Map, String, TryFromVal, Val, Vec,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct GameItem {
    pub item_id: String,
    pub owner: Address,
    pub metadata: BytesN<64>, // Hash or URI of item metadata
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct TradeOffer {
    pub item_id: String,
    pub seller: Address,
    pub price: i128,
    pub is_active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Auction {
    pub item_id: String,
    pub seller: Address,
    pub highest_bid: i128,
    pub highest_bidder: Address,
    pub end_time: u64,
    pub is_active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct TradeHistory {
    pub seller: Address,
    pub buyer: Address,
    pub price: i128,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NotFound = 3, // Generic not found
    ItemNotFound = 4,
    TradeOfferNotFound = 5,
    AuctionNotFound = 6,
    InvalidCaller = 7, // Caller is not authorized (e.g., not owner)
    NotItemOwner = 8,
    OfferNotActive = 9,
    AuctionNotActive = 10,
    AuctionAlreadyEnded = 11,
    AuctionNotEnded = 12,
    BidTooLow = 13,
    InsufficientFunds = 14, // Although token transfer handles this, good for clarity
    InvalidAmount = 15,     // e.g., price <= 0
    InvalidEndTime = 16,    // e.g., end time in the past
    CannotBidOnOwnAuction = 17,
    NoBidsPlaced = 18,
    TradeAlreadyExists = 19,
    AuctionAlreadyExists = 20,
    ItemAlreadyListed = 21, // Item is already in a trade or auction
    InvalidMetadata = 22,   // Metadata length issue (though BytesN handles size)
    ItemNotListed = 23,
    AuctionStillRunning = 24,
    BidAlreadyPlaced = 25,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum DataKey {
    Admin,
    Items,
    TradeOffer,
    Listed,
    Auction,
    TradeHistory,
}
