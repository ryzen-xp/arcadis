use soroban_sdk::{Address, Env, Map, String, Symbol, Vec};
use crate::escrow::{deposit_escrow, withdraw_escrow};
use crate::utils::{DataKey, Error, GameItem, TradeHistory, TradeOffer};

pub fn create_trade_offer(
    e: Env,
    item_id: String,
    seller: Address,
    price: i128,
) -> Result<(), Error> {
    seller.require_auth();

    if price <= 0 {
        return Err(Error::InvalidAmount);
    }

    let items_key = DataKey::Items;
    let trade_offer_key = DataKey::TradeOffer;
    let listed_key = DataKey::Listed;

    let items: Map<String, GameItem> = match e.storage().instance().get(&items_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut trade_offer: Map<String, TradeOffer> = match e.storage().instance().get(&trade_offer_key)
    {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut listed: Map<String, bool> = match e.storage().instance().get(&listed_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    if items.get(item_id.clone()).unwrap().owner != seller {
        return Err(Error::NotItemOwner);
    }
    if listed.get(item_id.clone()).unwrap() {
        return Err(Error::ItemAlreadyListed);
    }

    if trade_offer.get(item_id.clone()).unwrap().is_active {
        return Err(Error::ItemAlreadyListed);
    }

    let instance_trade_offer = TradeOffer {
        item_id: item_id.clone(),
        seller,
        price,
        is_active: true,
    };

    trade_offer.set(item_id.clone(), instance_trade_offer.clone());
    listed.set(item_id.clone(), true);

    e.storage().instance().set(&trade_offer_key, &trade_offer);
    e.storage().instance().set(&listed_key, &listed);

    e.events().publish(
        (Symbol::new(&e, "created_trade_offer"), item_id),
        instance_trade_offer,
    );

    Ok(())
}

pub fn execute_trade(e: Env, item_id: String, buyer: Address) -> Result<(), Error> {
    buyer.require_auth();

    let items_key = DataKey::Items;
    let trade_offer_key = DataKey::TradeOffer;
    let listed_key = DataKey::Listed;
    let trade_history_key = DataKey::TradeHistory;

    let mut histories: Map<String, Vec<TradeHistory>> =
        match e.storage().instance().get(&trade_history_key) {
            Some(x) => x,
            None => Map::new(&e),
        };

    let mut items: Map<String, GameItem> = match e.storage().instance().get(&items_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut trade_offer: Map<String, TradeOffer> =
        match e.storage().instance().get(&trade_offer_key) {
            Some(x) => x,
            None => Map::new(&e),
        };

    let mut listed: Map<String, bool> = match e.storage().instance().get(&listed_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut item = match items.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::NotInitialized),
    };

    let offer = match trade_offer.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::NotInitialized),
    };

    if !listed.get(item_id.clone()).unwrap_or(false) {
        return Err(Error::NotInitialized);
    }
    if offer.seller == buyer {
        return Err(Error::InvalidCaller);
    }
    if !offer.is_active {
        return Err(Error::OfferNotActive);
    }

    //  Deposit buyer's payment into escrow
    deposit_escrow(&e, buyer.clone(), offer.price)?;

    //  Transfer item to buyer
    item.owner = buyer.clone();
    items.set(item_id.clone(), item.clone());
    e.storage().instance().set(&items_key, &items);

    //  Remove the trade offer and listing
    trade_offer.remove(item_id.clone());
    e.storage().instance().set(&trade_offer_key, &trade_offer);

    listed.set(item_id.clone(), false);
    e.storage().instance().set(&listed_key, &listed);

    //  Transfer escrowed funds to seller
    withdraw_escrow(&e, buyer.clone(), offer.price)?;
    deposit_escrow(&e, offer.seller.clone(), offer.price)?;

    // TRADE HISTORY
    let history_entry = TradeHistory {
        seller: offer.seller.clone(),
        buyer: buyer.clone(),
        price: offer.price,
        timestamp: e.ledger().timestamp(),
    };

    let mut item_histories = histories.get(item_id.clone()).unwrap_or(Vec::new(&e));
    item_histories.push_back(history_entry);
    histories.set(item_id.clone(), item_histories);

    e.storage().instance().set(&trade_history_key, &histories);

    // EVENTS
    e.events()
        .publish((Symbol::new(&e, "trade_executed"), buyer.clone()), item);

    e.events().publish(
        (Symbol::new(&e, "Trade_History_Updated"), item_id.clone()),
        (offer.seller, buyer, offer.price, e.ledger().timestamp()),
    );

    Ok(())
}

pub fn cancel_trade_offer(e: Env, item_id: String, seller: Address) -> Result<(), Error> {
    seller.require_auth();

    let items_key = DataKey::Items;
    let trade_offer_key = DataKey::TradeOffer;
    let listed_key = DataKey::Listed;

    let items: Map<String, GameItem> = match e.storage().instance().get(&items_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let mut trade_offer: Map<String, TradeOffer> =
        match e.storage().instance().get(&trade_offer_key) {
            Some(x) => x,
            None => Map::new(&e),
        };

    let mut listed: Map<String, bool> = match e.storage().instance().get(&listed_key) {
        Some(x) => x,
        None => Map::new(&e),
    };

    let item = match items.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::NotInitialized),
    };

    let offer = match trade_offer.get(item_id.clone()) {
        Some(x) => x,
        None => return Err(Error::NotInitialized),
    };

    if item.owner != seller {
        return Err(Error::NotItemOwner);
    }

    if !listed.get(item_id.clone()).unwrap_or(false) {
        return Err(Error::ItemNotListed);
    }

    if !offer.is_active {
        return Err(Error::OfferNotActive);
    }

    trade_offer.remove(item_id.clone());
    listed.set(item_id.clone(), false);

    e.storage().instance().set(&trade_offer_key, &trade_offer);
    e.storage().instance().set(&listed_key, &listed);

    e.events()
        .publish((Symbol::new(&e, "canceled_trade_offer"), seller), item_id);

    Ok(())
}
