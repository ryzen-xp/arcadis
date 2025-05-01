//! src/events.rs

extern crate alloc;
use alloc::{string::ToString, vec::Vec as StdVec};

use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env, FromVal, IntoVal, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct GameEvent {
    pub event_id: String,
    pub player: Address,
    pub event_type: Symbol, 
    pub metadata: BytesN<64>,
    pub timestamp: u64,
}

const EVENT_KEY: &str = "EVENTS";

pub fn log_event(
    env: &Env,
    player: Address,
    event_type: Symbol, 
    metadata: BytesN<64>,
) -> String {
    player.require_auth();
    let timestamp = env.ledger().timestamp();

    // Convert Address to raw 32-byte array
    let addr_bytesn: BytesN<32> = BytesN::from_val(env, &player.to_val());
    let addr_array: [u8; 32] = addr_bytesn.into();

    // Hex-encode address and append '_' + timestamp
    let mut buf: StdVec<u8> = StdVec::with_capacity(32 * 2 + 1 + 20);
    for &b in addr_array.iter() {
        let hi = b >> 4;
        let lo = b & 0xF;
        buf.push(if hi < 10 { b'0' + hi } else { b'a' + (hi - 10) });
        buf.push(if lo < 10 { b'0' + lo } else { b'a' + (lo - 10) });
    }
    buf.push(b'_');
    let ts_std: alloc::string::String = timestamp.to_string();
    buf.extend(ts_std.as_bytes());

    // Build Soroban String from raw bytes
    let event_id = String::from_bytes(env, &buf);

    let event = GameEvent {
        event_id: event_id.clone(),
        player,
        event_type, 
        metadata,
        timestamp,
    };

    let mut events: Vec<GameEvent> = env.storage().persistent()
        .get(&String::from_str(env, EVENT_KEY))
        .unwrap_or(Vec::new(env));
    events.push_back(event.clone());
    env.storage().persistent()
        .set(&String::from_str(env, EVENT_KEY), &events);

    event_id
}

pub fn get_event_log(
    env: &Env,
    player: Option<Address>,
    region_id: Option<String>,
) -> Vec<GameEvent> {
    let events: Vec<GameEvent> = env.storage().persistent()
        .get(&String::from_str(env, EVENT_KEY))
        .unwrap_or(Vec::new(env));

    let mut filtered = Vec::new(env);
    for e in events.iter() {
        let player_match = player.as_ref().map_or(true, |p| e.player == *p);
        let region_match = region_id.as_ref().map_or(true, |r| {
            let region_bytesn: BytesN<32> = BytesN::from_val(env, &r.to_val());
            let region_array: [u8; 32] = region_bytesn.into();
            let region_bytes = Bytes::from_slice(env, &region_array);
            let metadata_bytes: Bytes = e.metadata.clone().into();
            contains_bytes(&metadata_bytes, &region_bytes)
        });
        if player_match && region_match {
            filtered.push_back(e.clone());
        }
    }

    filtered
}

// Helper: check byte containment
fn contains_bytes(haystack: &Bytes, needle: &Bytes) -> bool {
    let haystack_len = haystack.len();
    let needle_len = needle.len();
    if needle_len == 0 || haystack_len < needle_len {
        return false;
    }
    for i in 0..=haystack_len - needle_len {
        let mut found = true;
        for j in 0..needle_len {
            if haystack.get(i + j) != needle.get(j) {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}