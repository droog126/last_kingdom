use bevy::prelude::*;

pub struct PhysicsPlugin;


pub const TRANSFORM_SYNC_STAGE: &'static str = "rapier::transform_sync_stage";

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
     
    }

}
