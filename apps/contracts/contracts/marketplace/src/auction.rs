#![no_std]
use soroban_sdk::{Address, Env, Map, String, Symbol};

use crate::utils::{Auction, DataKey, Error, GameItem};

pub fn start_auction(
    e: Env,
    item_id: String,
    seller: Address,
    base_price: i128,
    duration: u64,
) -> Result<(), Error> {
    seller.require_auth();

    if duration == 0 {
        return Err(Error::InvalidEndTime);
    }

    let auction_key = DataKey::Auction;
    let listed_key = DataKey::Listed;
    let items_key = DataKey::Items;

    let items: Map<String, GameItem> = match e.storage().instance().get(&items_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut auctions: Map<String, Auction> = match e.storage().instance().get(&auction_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut listed: Map<String, bool> = match e.storage().instance().get(&listed_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let item = match items.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::ItemNotFound),
    };

    if item.owner != seller {
        return Err(Error::NotItemOwner);
    }

    if !listed.get(item_id.clone()).unwrap_or(false) {
        return Err(Error::ItemNotListed);
    }

    if let Some(existing_auction) = auctions.get(item_id.clone()) {
        if existing_auction.is_active {
            return Err(Error::AlreadyInitialized);
        }
    }

    let auction_end_time = e.ledger().timestamp() + duration;

    let new_auction = Auction {
        item_id: item_id.clone(),
        seller: seller.clone(),
        end_time: auction_end_time,
        highest_bid: base_price,
        highest_bidder: seller.clone(),
        is_active: true,
    };

    auctions.set(item_id.clone(), new_auction.clone());
    e.storage().instance().set(&auction_key, &auctions);

    listed.set(item_id.clone(), true);
    e.storage().instance().set(&listed_key, &listed);

    e.events()
        .publish((Symbol::new(&e, "Auction_started"), seller), new_auction);

    Ok(())
}

pub fn place_bid(e: Env, item_id: String, bidder: Address, bid: i128) -> Result<(), Error> {
    bidder.require_auth();

    if bid <= 0 {
        return Err(Error::InvalidAmount);
    }

    let auction_key = DataKey::Auction;

    let mut auctions: Map<String, Auction> = match e.storage().instance().get(&auction_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut existing_auction = match auctions.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::AuctionNotFound),
    };

    let current_time = e.ledger().timestamp();

    if !existing_auction.is_active {
        return Err(Error::AuctionNotActive);
    }
    if existing_auction.seller == bidder {
        return Err(Error::InvalidCaller);
    }
    if existing_auction.end_time < current_time {
        return Err(Error::AuctionAlreadyEnded);
    }
    if existing_auction.highest_bid >= bid {
        return Err(Error::BidTooLow);
    }

    let pre_bid = existing_auction.highest_bid;
    let pre_bidder = existing_auction.highest_bidder;

    existing_auction.highest_bid = bid;
    existing_auction.highest_bidder = bidder.clone();

    auctions.set(item_id.clone(), existing_auction);

    e.storage().instance().set(&auction_key, &auctions);

    e.events().publish(
        (Symbol::new(&e, "placed_new_bid"), item_id),
        (pre_bid, bid, pre_bidder, bidder),
    );

    Ok(())
}

pub fn claim_bid(e: Env, item_id: String, claimer: Address) -> Result<(), Error> {
    claimer.require_auth();

    let auction_key = DataKey::Auction;
    let items_key = DataKey::Items;

    let mut auctions: Map<String, Auction> = match e.storage().instance().get(&auction_key) {
        Some(x) => x,
        None => return Err(Error::AuctionNotFound),
    };

    let mut items: Map<String, GameItem> = match e.storage().instance().get(&items_key) {
        Some(x) => x,
        None => return Err(Error::ItemNotFound),
    };

    let mut auction = match auctions.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::AuctionNotFound),
    };

    let current_time = e.ledger().timestamp();

    if !auction.is_active {
        return Err(Error::AuctionNotActive);
    }
    if auction.end_time > current_time {
        return Err(Error::AuctionStillRunning);
    }
    if auction.highest_bidder != claimer {
        return Err(Error::InvalidCaller);
    }

    let mut item = match items.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::ItemNotFound),
    };

    // Transfer ownership
    item.owner = claimer.clone();

    // Update storage
    items.set(item_id.clone(), item);
    e.storage().instance().set(&items_key, &items);

    // Mark auction inactive
    auction.is_active = false;
    auctions.set(item_id.clone(), auction);
    e.storage().instance().set(&auction_key, &auctions);

    // Emit claim event
    e.events().publish(
        (Symbol::new(&e, "Auction_claimed"), item_id.clone()),
        claimer,
    );

    Ok(())
}

pub fn cancel_auction(e: Env, item_id: String, seller: Address) -> Result<(), Error> {
    seller.require_auth();

    let auction_key = DataKey::Auction;

    let mut auctions: Map<String, Auction> = match e.storage().instance().get(&auction_key) {
        Some(x) => x,
        None => return Err(Error::AuctionNotFound),
    };

    let auction = match auctions.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::AuctionNotFound),
    };

    if !auction.is_active {
        return Err(Error::AuctionNotActive);
    }
    if auction.seller != seller {
        return Err(Error::InvalidCaller);
    }

    if auction.highest_bidder != seller {
        return Err(Error::BidAlreadyPlaced);
    }

    auctions.remove(item_id.clone());
    e.storage().instance().set(&auction_key, &auctions);

    // Emit
    e.events().publish(
        (Symbol::new(&e, "Auction_cancelled"), item_id.clone()),
        seller,
    );

    Ok(())
}
