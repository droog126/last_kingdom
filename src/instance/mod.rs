use bevy::prelude::Component;

pub mod player;
pub mod snake;
pub mod utils;

// instanceTag

#[derive(Clone, Debug, PartialEq, Copy, Hash, std::cmp::Eq)]
pub enum InstanceType {
    Player,
    Snake,
    Wall,
}

#[derive(Debug, Clone)]
pub enum InstanceCamp {
    Neutral,
    Hostile,
    Friendly,
    Team { team_id: u32 },
}

#[derive(Clone, Debug, PartialEq, Copy, Hash, std::cmp::Eq)]
pub enum CollisionType {
    Static,
    Scope,
    Instance,
}

#[derive(Component, Debug, Clone)]
pub struct InstanceTypeValue {
    pub value: InstanceType,
}

#[derive(Component, Debug, Clone)]
pub struct InstanceCampValue {
    pub value: InstanceCamp,
}

#[derive(Component, Debug, Clone)]

pub struct CollisionTypeValue {
    pub value: CollisionType,
}
