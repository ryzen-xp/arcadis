#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, BytesN, String, Vec};

mod save;
mod world;
mod events;

#[contract]
pub struct GameStateContract;

#[contractimpl]
impl GameStateContract {
    pub fn save_progress(env: Env, player: Address, data: BytesN<64>) -> BytesN<32> {
        save::save_progress(&env, player, data)
    }

    pub fn load_progress(env: Env, player: Address, timestamp: Option<u64>) -> Option<save::SavePoint> {
        save::load_progress(&env, player, timestamp)
    }

    pub fn update_world_state(env: Env, region_id: String, state_data: BytesN<64>) {
        world::update_world_state(&env, region_id, state_data)
    }

    pub fn log_event(env: Env, player: Address, event_type: String, metadata: BytesN<64>) -> String {
        events::log_event(&env, player, event_type, metadata)
    }

    pub fn get_event_log(env: Env, player: Option<Address>, region_id: Option<String>) -> Vec<events::GameEvent> {
        events::get_event_log(&env, player, region_id)
    }
}