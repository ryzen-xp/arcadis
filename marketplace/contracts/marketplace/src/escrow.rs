#![no_std]
use soroban_sdk::{token, Address, Env, Map};

use crate::utils::{DataKey, Error, EscrowBalance};

pub fn internal_get_token(e: &Env) -> Result<Address, Error> {
    let token_key = DataKey::TokenAddress;

    match e.storage().instance().get(&token_key) {
        Some(addr) => Ok(addr),
        None => Err(Error::NotInitialized),
    }
}

pub fn deposit_escrow(e: &Env, user: Address, amount: i128) -> Result<(), Error> {
    let escrow_key = DataKey::Escrow;

    let mut escrows: Map<Address, EscrowBalance> = e
        .storage()
        .instance()
        .get(&escrow_key)
        .unwrap_or(Map::new(e));

    let mut user_escrow = escrows
        .get(user.clone())
        .unwrap_or(EscrowBalance { balance: 0 });

    let token_address = internal_get_token(e)?;
    let client = token::Client::new(e, &token_address);
    client.transfer(&user, &e.current_contract_address(), &amount);

    user_escrow.balance += amount;
    escrows.set(user.clone(), user_escrow);

    e.storage().instance().set(&escrow_key, &escrows);

    Ok(())
}

pub fn withdraw_escrow(e: &Env, user: Address, amount: i128) -> Result<(), Error> {
    let escrow_key = DataKey::Escrow;

    let mut escrows: Map<Address, EscrowBalance> = e
        .storage()
        .instance()
        .get(&escrow_key)
        .unwrap_or(Map::new(e));

    let mut user_escrow = escrows
        .get(user.clone())
        .unwrap_or(EscrowBalance { balance: 0 });

    if user_escrow.balance < amount {
        return Err(Error::InsufficientFunds);
    }

    user_escrow.balance -= amount;
    escrows.set(user.clone(), user_escrow);
    e.storage().instance().set(&escrow_key, &escrows);

    let token_address = internal_get_token(e)?;
    let client = token::Client::new(e, &token_address);
    client.transfer(&e.current_contract_address(), &user, &amount);

    Ok(())
}
