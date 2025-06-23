use crate::game::world::{World, WorldManager};

pub mod world;
pub mod player;
pub mod entity;
mod math;

/// Ideas
/// - client is not directly related to player
/// - player is entity with some addiytional data
/// - client should be able to access world, example: viewer to look at world content
/// - maybe client should have permissions: regular client should have attached player, admin cleint can freely access API


pub struct Game {
    pub world_manager: WorldManager,
}

impl Game {
    pub async fn new() -> Self {
        let world_manager = WorldManager::run().await;
        
        Self { world_manager }
    }
}