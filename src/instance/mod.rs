use bevy::prelude::Component;

pub mod player;
pub mod snake;
pub mod utils;

#[derive(Debug, Clone)]
pub enum InstanceType {
    Player,
    Snake,
}

#[derive(Debug, Clone)]
pub enum InstanceCamp {
    Neutral,
    Hostile,
    Friendly,
    Team { team_id: u32 },
}

#[derive(Component, Debug, Clone)]
pub struct InstanceCategory {
    pub type_: InstanceType,
    pub camp: InstanceCamp,
}
