#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec};

pub use auction::{cancel_auction, claim_bid, place_bid, start_auction};
pub use history::get_trade_history;
pub use trade::{cancel_trade_offer, create_trade_offer, execute_trade};
use utils::DataKey;
pub use utils::{Error, TradeHistory};

mod auction;
mod escrow;
mod history;
mod trade;
mod utils;

#[contract]
pub struct ContractMarketplace;

#[contractimpl]
impl ContractMarketplace {
    pub fn initialize(e: Env, admin: Address, token: Address) -> Result<(), Error> {
        let admin_key = DataKey::Admin;
        let token_key = DataKey::TokenAddress;
        if e.storage().instance().has(&admin_key) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();

        e.storage().instance().set(&admin_key, &admin);
        e.storage().instance().set(&token_key, &token);

        Ok(())
    }
    // set nnew token address
    pub fn set_token_address(e: Env, admin: Address, new_token: Address) -> Result<(), Error> {
        admin.require_auth();
        let admin_key = DataKey::Admin;
        let token_key = DataKey::TokenAddress;
        let current_admin: Address = e
            .storage()
            .instance()
            .get(&admin_key)
            .ok_or(Error::NotInitialized)?;
        if current_admin != admin {
            return Err(Error::InvalidCaller);
        }
        e.storage().instance().set(&token_key, &new_token);
        e.events()
            .publish((Symbol::new(&e, "new_token_address_set"), admin), new_token);

        Ok(())
    }
    // List an item for sale
    pub fn list_for_sale(
        e: Env,
        item_id: String,
        seller: Address,
        price: i128,
    ) -> Result<(), utils::Error> {
        create_trade_offer(e, item_id, seller, price)
    }

    // Delist an item
    pub fn delist_from_sale(e: Env, item_id: String, seller: Address) -> Result<(), utils::Error> {
        cancel_trade_offer(e, item_id, seller)
    }

    // Finalize a peer-to-peer trade
    pub fn p2p_execute_trade(e: Env, item_id: String, buyer: Address) -> Result<(), utils::Error> {
        execute_trade(e, item_id, buyer)
    }

    // Start an auction
    pub fn start_auction(
        e: Env,
        item_id: String,
        seller: Address,
        starting_bid: i128,
        duration: u64,
    ) -> Result<(), utils::Error> {
        start_auction(e, item_id, seller, starting_bid, duration)
    }

    // Place a bid
    pub fn place_auction_bid(
        e: Env,
        item_id: String,
        bidder: Address,
        bid_amount: i128,
    ) -> Result<(), utils::Error> {
        place_bid(e, item_id, bidder, bid_amount)
    }

    // Claim the winning auction item
    pub fn claim_auction_bid(
        e: Env,
        item_id: String,
        claimer: Address,
    ) -> Result<(), utils::Error> {
        claim_bid(e, item_id, claimer)
    }

    // Cancel an auction
    pub fn cancel_auction(e: Env, item_id: String, seller: Address) -> Result<(), utils::Error> {
        cancel_auction(e, item_id, seller)
    }

    // Retrieve trade history
    pub fn get_trade_history(e: Env, item_id: String) -> Result<Vec<TradeHistory>, Error> {
        get_trade_history(e, item_id)
    }
}
