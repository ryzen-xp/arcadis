use soroban_sdk::{Env, Map, String, Vec};
use crate::utils::{DataKey, Error, TradeHistory};

pub fn get_trade_history(e: Env, item_id: String) -> Result<Vec<TradeHistory>, Error> {
    let trade_history_key = DataKey::TradeHistory;

    let histories: Map<String, Vec<TradeHistory>> =
        match e.storage().instance().get(&trade_history_key) {
            Some(x) => x,
            None => return Err(Error::NotInitialized),
        };

    let item_histories = match histories.get(item_id) {
        Some(x) => x,
        None => Vec::new(&e),
    };

    Ok(item_histories)
}
