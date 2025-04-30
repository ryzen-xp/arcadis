use soroban_sdk::{contracttype, Env, String, BytesN, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct WorldState {
    pub region_id: String,
    pub state_data: BytesN<64>,
    pub last_updated: u64,
}

const WORLD_KEY: &str = "WORLD";

pub fn update_world_state(env: &Env, region_id: String, state_data: BytesN<64>) {
    let timestamp = env.ledger().timestamp();
    
    let world_state = WorldState {
        region_id: region_id.clone(),
        state_data,
        last_updated: timestamp,
    };
    
    let mut world_states: Vec<WorldState> = env.storage().persistent().get(&WORLD_KEY).unwrap_or(Vec::new(&env));
    
    if let Some(index) = world_states.iter().position(|ws| ws.region_id == region_id) {
        world_states.set(index as u32, world_state);
    } else {
        world_states.push_back(world_state);
    }
    
    env.storage().persistent().set(&WORLD_KEY, &world_states);
}